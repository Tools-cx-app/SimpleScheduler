mod fmt;

use std::path::PathBuf;

use anyhow::Result;
use log::info;

use crate::{
    cpu::{freqs::CpuFreqs, governors::CpuGovernors, misc::read_policy},
    files_handler::FilesHandler,
    framework::{
        Error,
        config::Config,
        scheduler::{topapps::TopWatcher, wake::Wake},
    },
    msic::get_process_name_by_pid,
};

struct SimpleSchedulerData {
    topapps: TopWatcher,
}

#[derive(Clone, Debug)]
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
    policys: Vec<PathBuf>,
    files_handler: FilesHandler,
    wake: Wake,
    mode: Option<SimpleSchedulerMode>,
}

impl Looper {
    pub fn new(c: Config) -> Result<Self> {
        let policys = read_policy()?;
        Ok(Self {
            config: c,
            data: SimpleSchedulerData {
                topapps: TopWatcher::new(),
            },
            last: LastCache {
                topapps: Vec::new(),
            },
            policys,
            files_handler: FilesHandler::new(),
            wake: Wake::new(),
            mode: None,
        })
    }

    pub fn enter_looper(&mut self) -> Result<()> {
        let mut updated = false;
        loop {
            self.reflash_governors(false)?;
            self.reflash_data();
            if !self.check_all() {
                continue;
            }
            if !self.wake.info() {
                self.mode = Some(SimpleSchedulerMode::Powersave);
                self.write_cpu_freqs()?;
                self.reflash_governors(false)?;
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

            self.mode = Some(mode.clone());

            if name != name_cache && is_list {
                self.reflash_governors(true)?;
                self.write_cpu_freqs()?;
                info!("New buffer for {name}(mode: {mode})");
                self.last.topapps = self.data.topapps.pids();
            }
        }
    }

    fn reflash_governors(&mut self, game: bool) -> Result<()> {
        for i in self.policys.clone() {
            let governors = CpuGovernors::new(i)?;
            if game {
                governors.auto_write_games(&mut self.files_handler)?;
            } else {
                governors.auto_write(&mut self.files_handler)?;
            }
        }
        Ok(())
    }

    fn write_cpu_freqs(&mut self) -> Result<()> {
        for i in self.policys.clone() {
            let cpus = CpuFreqs::new(i)?;
            let (target_max_freq, target_min_freq) = {
                let config = self.config.config().freqs.clone();
                match self.mode.clone().unwrap_or(SimpleSchedulerMode::Bablance) {
                    SimpleSchedulerMode::Powersave => config.powersave,
                    SimpleSchedulerMode::Bablance => config.balance,
                    SimpleSchedulerMode::Performance => config.performance,
                }
            };
            cpus.write_freq(target_max_freq, target_min_freq, &mut self.files_handler)?;
        }

        Ok(())
    }

    fn reflash_data(&mut self) {
        self.data.topapps.info();
    }

    fn check_all(&self) -> bool {
        if self.data.topapps.visible_freeform_window() {
            return false;
        }
        if self.data.topapps.pids().is_empty() {
            return false;
        }

        true
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
