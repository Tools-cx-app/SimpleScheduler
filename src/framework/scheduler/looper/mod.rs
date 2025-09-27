mod fmt;

use anyhow::Result;
use log::info;

use crate::{
    framework::{Error, config::Config, scheduler::topapps::TopWatcher},
    msic::get_process_name_by_pid,
};

struct SimpleSchedulerData {
    topapps: TopWatcher,
}

enum SimpleSchedulerMode {
    Powersave,
    Bablance,
    Performance,
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
            self.reflash_topapps();
            if self.data.topapps.visible_freeform_window() {
                continue;
            }

            let pid = self.data.topapps.pids()[0];
            let name = get_process_name_by_pid(pid)?;
            let mode = self.list_include_target(&name)?;
        }
    }

    fn reflash_topapps(&mut self) {
        self.data.topapps.info();
    }

    fn list_include_target(&mut self, target: &str) -> Result<SimpleSchedulerMode, Error> {
        let config = &self.config.config().config;

        if config.bablance.contains(target) {
            return Ok(SimpleSchedulerMode::Bablance);
        } else if config.powersave.contains(target) {
            return Ok(SimpleSchedulerMode::Powersave);
        } else if config.performance.contains(target) {
            return Ok(SimpleSchedulerMode::Performance);
        }

        match config.general.as_str() {
            "powersave" => Ok(SimpleSchedulerMode::Powersave),
            "bablance" => Ok(SimpleSchedulerMode::Bablance),
            "performance" => Ok(SimpleSchedulerMode::Performance),
            _ => Err(Error::ConfigParse("general option")),
        }
    }
}
