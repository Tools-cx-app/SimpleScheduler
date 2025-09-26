use anyhow::Result;

use crate::framework::{config::Config, scheduler::topapps::TopWatcher};

struct SimpleSchedulerData {
    topapps: TopWatcher,
}

pub struct Looper {
    config: Config,
    data: SimpleSchedulerData,
}

impl Looper {
    pub fn new(c: Config) -> Self {
        Self {
            config: c,
            data: SimpleSchedulerData {
                topapps: TopWatcher::new(),
            },
        }
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        loop {
            if self.data.topapps.visible_freeform_window() {
                continue;
            }
            self.reflash_topapps();
        }
    }

    fn reflash_topapps(&mut self) {
        self.data.topapps.info();
    }
}
