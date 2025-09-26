use anyhow::Result;

use crate::framework::{config::data::ConfigData, scheduler::topapps::TopWatcher};

struct SimpleSchedulerData {
    topapps: TopWatcher,
}

pub struct Looper {
    config: ConfigData,
    data: SimpleSchedulerData,
}

impl Looper {
    pub fn new(c: ConfigData) -> Self {
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
