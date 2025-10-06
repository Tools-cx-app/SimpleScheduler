use std::{
    process::Command,
    time::{Duration, Instant},
};

const REFRESH_TIME: Duration = Duration::from_millis(500);

pub struct Wake {
    last_update: Instant,
    status: bool,
}

impl Wake {
    pub fn new() -> Self {
        Self {
            last_update: Instant::now(),
            status: bool::default(),
        }
    }

    pub fn info(&mut self) -> bool {
        if self.last_update.elapsed() > REFRESH_TIME {
            let dump = {
                let command = match Command::new("dumpsys").arg("power").output() {
                    Ok(o) => o,
                    Err(e) => {
                        log::debug!("Failed to dump power status: {e}");
                        return true;
                    }
                };
                if !command.status.success() {
                    log::debug!("Failed to dump power status");
                    return true;
                }
                String::from_utf8_lossy(&command.stdout).to_string()
            };

            self.status = Self::parse_info(&dump);
        }

        self.status
    }

    fn parse_info(dump: &str) -> bool {
        dump.contains("mWakefulness=Awake") || dump.contains("Display Power: state=ON")
    }
}
