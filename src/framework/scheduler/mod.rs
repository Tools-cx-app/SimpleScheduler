mod looper;
mod topapps;

use std::sync::mpsc::Receiver;

use anyhow::Result;

use crate::framework::{
    Error,
    config::{Config, data::ConfigData},
    scheduler::looper::Looper,
};

pub struct Scheduler {
    config: Option<Config>,
}

impl Scheduler {
    #[must_use]
    pub const fn new() -> Self {
        Self { config: None }
    }

    pub fn config(mut self, c: Config) -> Self {
        self.config = Some(c);
        self
    }

    pub fn start_run(self) -> Result<()> {
        let config = self.config.ok_or(Error::SchedulerMissing("Config"))?;

        Looper::new(config).enter_looper()
    }
}
