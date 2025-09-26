use std::time::{Duration, Instant};

use dumpsys_rs::Dumpsys;

const RETRY_TIME: Duration = Duration::from_secs(1);
const REFRESH_TIME: Duration = Duration::from_millis(500);

#[derive(Default)]
pub struct TopAppData {
    pub visible_freeform_window: bool,
    pub pids: Vec<i32>,
}

pub struct TopWatcher {
    dumper: Dumpsys,
    info: TopAppData,
    last_update: Instant,
}

impl TopAppData {
    fn new(dump: &str) -> Self {
        let pids = Self::parse_pid(dump);
        let visible_freeform_window = dump.contains("freeform")
            || dump.contains("FlexibleTaskCaptionView")
            || dump.contains("FlexibleTaskIndicatorView");

        Self {
            pids,
            visible_freeform_window,
        }
    }

    fn parse_pid(dump: &str) -> Vec<i32> {
        dump.lines()
            .filter(|l| l.contains("Session{"))
            .filter_map(|l| l.split_whitespace().nth(3))
            .filter_map(|s| s.split(':').next())
            .map(|p| p.trim().parse().unwrap())
            .collect()
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
}
