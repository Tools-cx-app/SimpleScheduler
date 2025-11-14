mod fmt;

use std::path::PathBuf;

use anyhow::Result;
use log::{debug, info};

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
pub enum SimpleSchedulerMode {
    Powersave,
    Balance,
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
            self.reflash_data();
            if !self.check_all() {
                continue;
            }
            if !self.wake.info() {
                self.mode = Some(SimpleSchedulerMode::Powersave);
                self.write_cpu_freqs()?;
                self.reflash_governors()?;
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

            self.mode = Some(mode.clone());
            debug!("current mode: {mode}");

            if name != name_cache && is_list {
                self.reflash_governors()?;
                self.write_cpu_freqs()?;
                info!("New buffer for {name}(mode: {mode})");
                self.last.topapps = self.data.topapps.pids();
            } else {
                self.write_cpu_freqs()?;
                self.reflash_governors()?;
            }

            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    fn reflash_governors(&mut self) -> Result<()> {
        for i in self.policys.clone() {
            let governors = CpuGovernors::new(
                i,
                self.mode.clone().unwrap(),
                self.config.config().cpu.governors.clone(),
            )?;
            governors.auto_write(&mut self.files_handler)?;
            debug!("write governors to cpu{} successful", governors.policy);
        }
        Ok(())
    }

    fn write_cpu_freqs(&mut self) -> Result<()> {
        for i in self.policys.clone() {
            let cpus = CpuFreqs::new(i)?;
            let freqs = {
                let config = self.config.config().cpu.clone();
                match self.mode.clone().unwrap_or(SimpleSchedulerMode::Balance) {
                    SimpleSchedulerMode::Powersave => config.powersave,
                    SimpleSchedulerMode::Balance => config.balance,
                    SimpleSchedulerMode::Performance => config.performance,
                }
            };
            cpus.write_freqs(&freqs.clone(), &mut self.files_handler)?;
            debug!("write freqs to cpu{} successful", cpus.policy);
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
            ret = SimpleSchedulerMode::Balance;
        } else if config.powersave.contains(target) {
            ret = SimpleSchedulerMode::Powersave;
        } else if config.performance.contains(target) {
            ret = SimpleSchedulerMode::Performance;
        } else {
            ret = match config.general.as_str() {
                "powersave" => SimpleSchedulerMode::Powersave,
                "balance" => SimpleSchedulerMode::Balance,
                "performance" => SimpleSchedulerMode::Performance,
                _ => return Err(Error::ConfigParse("general option")),
            }
        }

        Ok((ret, true))
    }
}
