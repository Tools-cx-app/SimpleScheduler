use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

use crate::{files_handler::FilesHandler, framework::scheduler::looper::SimpleSchedulerMode};

pub struct CpuGovernors {
    pub policy: i32,
    path: PathBuf,
    mode: SimpleSchedulerMode,
    pub governors: Vec<String>,
}

impl CpuGovernors {
    pub fn new<P>(path: P, mode: SimpleSchedulerMode) -> Result<Self>
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
            governors,
        })
    }

    pub fn auto_write(&self, files_handler: &mut FilesHandler) -> Result<()> {
        match self.mode {
            SimpleSchedulerMode::Performance => {
                files_handler
                    .write_with_handler(self.path.join("scaling_governor"), "performance")?;
                return Ok(());
            }
            SimpleSchedulerMode::Powersave => {
                files_handler
                    .write_with_handler(self.path.join("scaling_governor"), "powersave")?;
                return Ok(());
            }
            SimpleSchedulerMode::Balance => {}
        }
        if self.governors.contains(&"walt".to_string()) {
            files_handler.write_with_handler(self.path.join("scaling_governor"), "walt")?;
        } else {
            files_handler.write_with_handler(self.path.join("scaling_governor"), "schedutil")?;
        }

        Ok(())
    }
}
