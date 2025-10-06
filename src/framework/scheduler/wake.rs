use std::time::{Duration, Instant};

use dumpsys_rs::Dumpsys;

const RETRY_TIME: Duration = Duration::from_secs(1);
const REFRESH_TIME: Duration = Duration::from_millis(500);

pub struct Wake {
    dumper: Dumpsys,
    last_update: Instant,
    status: bool,
}

impl Wake {
    pub fn new() -> Self {
        let dumper = loop {
            match Dumpsys::new("power") {
                Some(s) => break s,
                None => std::thread::sleep(RETRY_TIME),
            }
        };
        Self {
            dumper,
            last_update: Instant::now(),
            status: bool::default(),
        }
    }

    pub fn info(&mut self) -> bool {
        if self.last_update.elapsed() > REFRESH_TIME {
            let dump = loop {
                match self.dumper.dump(&[""]) {
                    Ok(dump) => break dump,
                    Err(e) => {
                        log::error!("Failed to dump power: {e}, retrying");
                        std::thread::sleep(RETRY_TIME);
                    }
                }
            };

            self.status = Self::parse_info(&dump);
        }

        self.status
    }

    fn parse_info(dump: &str) -> bool {
        dump.contains("mWakefulness=Awake") || dump.contains("Display Power: state=ON")
    }
}
