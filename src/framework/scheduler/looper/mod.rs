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

struct LastCache {
    topapps: Vec<i32>,
}
pub struct Looper {
    config: Config,
    data: SimpleSchedulerData,
    last: LastCache,
}

impl Looper {
    pub fn new(c: Config) -> Self {
        Self {
            config: c,
            data: SimpleSchedulerData {
                topapps: TopWatcher::new(),
            },
            last: LastCache {
                topapps: Vec::new(),
            },
        }
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        let mut updated = false;
        loop {
            self.reflash_topapps();
            if self.data.topapps.visible_freeform_window() {
                continue;
            }
            if self.data.topapps.pids().is_empty() {
                continue;
            }
            if !updated {
                self.last.topapps = self.data.topapps.pids();
                updated = true;
            }
            
            let pid = self.data.topapps.pids()[0];
            let pid_cache = self.last.topapps[0];
            let name = get_process_name_by_pid(pid)?;
            let name_cache = get_process_name_by_pid(pid_cache)?;
            let (mode, is_list) = self.list_include_target(&name)?;
            if name != name_cache && is_list {
                info!("New buffer for {name}(mode: {mode})");
                self.last.topapps = self.data.topapps.pids();
            }
        }
    }

    fn reflash_topapps(&mut self) {
        self.data.topapps.info();
    }

    fn list_include_target(&mut self, target: &str) -> Result<(SimpleSchedulerMode, bool), Error> {
        let config = &self.config.config().config;
        let ret;

        if config.balance.contains(target) {
            ret = SimpleSchedulerMode::Bablance;
        } else if config.powersave.contains(target) {
            ret = SimpleSchedulerMode::Powersave;
        } else if config.performance.contains(target) {
            ret = SimpleSchedulerMode::Performance;
        } else {
            ret = match config.general.as_str() {
                "powersave" => SimpleSchedulerMode::Powersave,
                "bablance" => SimpleSchedulerMode::Bablance,
                "performance" => SimpleSchedulerMode::Performance,
                _ => return Err(Error::ConfigParse("general option")),
            }
        }

        Ok((ret, true))
    }
}
