mod looper;
mod topapps;

use anyhow::Result;

use crate::framework::{Error, config::data::ConfigData, scheduler::looper::Looper};

pub struct Scheduler {
    config: Option<ConfigData>,
}

impl Scheduler {
    #[must_use]
    pub fn new() -> Self {
        Self { config: None }
    }

    pub fn config(mut self, c: ConfigData) -> Self {
        self.config = Some(c);
        self
    }

    pub fn start_run(self) -> Result<()> {
        let config = self.config.ok_or(Error::SchedulerMissing("Config"))?;

        Looper::new(config).enter_looper()
    }
}
