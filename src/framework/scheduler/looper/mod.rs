mod fmt;

use anyhow::Result;
use log::{error, info};

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
            if self.data.topapps.visible_freeform_window() {
                continue;
            }

            let pid = self.data.topapps.pids()[0];
            let name = get_process_name_by_pid(pid)?;
            let mode = self.list_include_target(&name)?;

            info!("New buffer for {name}, mode {mode}");
            self.reflash_topapps();
        }
    }

    fn reflash_topapps(&mut self) {
        self.data.topapps.info();
    }

    fn list_include_target(&mut self, target: &str) -> Result<SimpleSchedulerMode, Error> {
        let config = self.config.config().config.clone();
        if config.bablance.contains(target) {
            Ok(SimpleSchedulerMode::Bablance)
        } else if config.powersave.contains(target) {
            Ok(SimpleSchedulerMode::Powersave)
        } else if config.performance.contains(target) {
            Ok(SimpleSchedulerMode::Performance)
        } else {
            match config.general.as_str() {
                "powersave" => Ok(SimpleSchedulerMode::Powersave),
                "bablance" => Ok(SimpleSchedulerMode::Bablance),
                "performance" => Ok(SimpleSchedulerMode::Performance),
                _ => {
                    return Err(Error::ConfigParse("general option"));
                }
            }
        }
    }
}
