use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::{
    files_handler::FilesHandler,
    framework::{config::data::Governors, scheduler::looper::SimpleSchedulerMode},
};

pub struct CpuGovernors {
    pub policy: i32,
    path: PathBuf,
    mode: SimpleSchedulerMode,
    config: Governors,
    pub governors: Vec<String>,
}

impl CpuGovernors {
    pub fn new<P>(path: P, mode: SimpleSchedulerMode, config: Governors) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref().to_path_buf();
        let file_name = path
            .file_name()
            .and_then(|s| s.to_str())
            .context("Invalid file name")?;

        let policy = file_name.get(6..).context("Invalid policy format")?;
        let policy = policy.parse::<i32>().context("Failed to parse policy")?;

        let governors_content = fs::read_to_string(path.join("scaling_available_governors"))
            .context("Failed to read governors")?;

        let mut governors = Vec::new();
        for i in governors_content.split_whitespace() {
            governors.push(i.to_string());
        }

        governors.sort_unstable();

        Ok(Self {
            policy,
            path,
            mode,
            config,
            governors,
        })
    }

    fn verify_governors<S>(&self, target: S) -> bool
    where
        S: Into<String>,
    {
        self.governors.contains(&target.into())
    }

    pub fn auto_write(&self, files_handler: &mut FilesHandler) -> Result<()> {
        match self.mode {
            SimpleSchedulerMode::Performance => {
                if !self.verify_governors(self.config.clone().performance) {
                    log::error!("governors option is error");
                    return Ok(());
                }
                files_handler.write_with_handler(
                    self.path.join("scaling_governor"),
                    self.config.clone().performance,
                )?
            }
            SimpleSchedulerMode::Powersave => {
                if !self.verify_governors(self.config.clone().powersave) {
                    log::error!("governors option is error");
                    return Ok(());
                }
                files_handler.write_with_handler(
                    self.path.join("scaling_governor"),
                    self.config.clone().powersave,
                )?
            }
            SimpleSchedulerMode::Balance => {
                if !self.verify_governors(self.config.clone().balance) {
                    log::error!("governors option is error");
                    return Ok(());
                }
                files_handler.write_with_handler(
                    self.path.join("scaling_governor"),
                    self.config.clone().balance,
                )?
            }
        }

        Ok(())
    }
}
