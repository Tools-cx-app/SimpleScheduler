use std::time::{Duration, Instant};

use dumpsys_rs::Dumpsys;

const RETRY_TIME: Duration = Duration::from_secs(1);
const REFRESH_TIME: Duration = Duration::from_millis(500);

#[derive(Default)]
pub struct TopAppData {
    visible_freeform_window: bool,
    pids: Vec<i32>,
}

pub struct TopWatcher {
    dumper: Dumpsys,
    info: TopAppData,
    last_update: Instant,
}

impl TopAppData {
    fn new(dump: &str) -> Self {
        let pids = Self::parse_top_app(dump);
        let visible_freeform_window = dump.contains("freeform")
            || dump.contains("FlexibleTaskCaptionView")
            || dump.contains("FlexibleTaskIndicatorView");

        Self {
            visible_freeform_window,
            pids,
        }
    }

    fn parse_top_app(dump: &str) -> Vec<i32> {
        let focused_app_line = match dump
            .lines()
            .find(|line| line.trim().starts_with("mFocusedApp="))
        {
            Some(line) => line,
            None => return Vec::new(),
        };

        let package_name = match Self::extract_package_name(focused_app_line) {
            Some(name) => name,
            None => return Vec::new(),
        };

        // Try modern parser, if it fails, fall back to legacy parser.
        let pid = Self::parse_a16_format(dump, &package_name)
            .or_else(|| Self::parse_a15_format(dump, &package_name));

        pid.map_or_else(Vec::new, |p| vec![p])
    }

    fn extract_package_name(line: &str) -> Option<&str> {
        line.split_whitespace()
            .find(|p| p.contains('/'))?
            .split('/')
            .next()
    }

    // Modern Parser (Android 16+)
    // Parses the PID from the `WINDOW MANAGER WINDOWS` section.
    fn parse_a16_format(dump: &str, package_name: &str) -> Option<i32> {
        let mut in_target_window_section = false;
        for line in dump.lines() {
            if !in_target_window_section {
                if line.contains("Window #") && line.contains(package_name) {
                    in_target_window_section = true;
                }
            } else {
                if line.contains("mSession=") {
                    let session_part = line.split("mSession=").nth(1)?;
                    let content_start = session_part.find('{')? + 1;
                    let content_end = session_part.find('}')?;
                    let content = &session_part[content_start..content_end];
                    let pid_part = content.split_whitespace().nth(1)?;
                    let pid_str = pid_part.split(':').next()?;
                    return pid_str.parse::<i32>().ok();
                }

                if line.contains("Window #") {
                    return None;
                }
            }
        }
        None
    }

    // Legacy Parser (Android 15 and older)
    // Parses the PID from the `WINDOW MANAGER SESSIONS` section.
    fn parse_a15_format(dump: &str, package_name: &str) -> Option<i32> {
        let mut last_pid_found: Option<i32> = None;
        for line in dump.lines() {
            if line.starts_with("  Session Session{") {
                let content_start = line.find('{')? + 1;
                let content_end = line.find('}')?;
                let content = &line[content_start..content_end];
                let pid_part = content.split_whitespace().nth(1)?;
                let pid_str = pid_part.split(':').next()?;
                last_pid_found = pid_str.parse::<i32>().ok();
            }

            let trimmed_line = line.trim();
            if trimmed_line.starts_with("mPackageName=") {
                if let Some(pkg) = trimmed_line.split('=').nth(1) {
                    if pkg == package_name {
                        return last_pid_found;
                    }
                }
            }
        }
        None
    }
}

impl TopWatcher {
    pub fn new() -> Self {
        let dumper = loop {
            match Dumpsys::new("window") {
                Some(s) => break s,
                None => std::thread::sleep(RETRY_TIME),
            }
        };
        Self {
            last_update: Instant::now(),
            info: TopAppData::default(),
            dumper,
        }
    }

    pub fn info(&mut self) -> &TopAppData {
        if self.last_update.elapsed() > REFRESH_TIME {
            let dump = loop {
                match self.dumper.dump(&["visible-apps"]) {
                    Ok(dump) => break dump,
                    Err(e) => {
                        log::error!("Failed to dump windows: {e}, retrying");
                        std::thread::sleep(RETRY_TIME);
                    }
                }
            };

            self.info = TopAppData::new(&dump);
            self.last_update = Instant::now();
        }

        &self.info
    }

    #[allow(clippy::missing_const_for_fn)]
    pub fn visible_freeform_window(&self) -> bool {
        self.info.visible_freeform_window
    }

    pub fn pids(&self) -> Vec<i32> {
        self.info.pids.clone()
    }
}
