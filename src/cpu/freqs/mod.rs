use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use log::warn;

use crate::files_handler::FilesHandler;

pub struct CpuFreqs {
    pub policy: i32,
    path: PathBuf,
    pub freqs: Vec<isize>,
}

impl CpuFreqs {
    pub fn new<P>(path: P) -> Result<Self>
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

        let freqs_content = fs::read_to_string(path.join("scaling_available_frequencies"))
            .context("Failed to read frequencies")?;
        let mut freqs: Vec<isize> = freqs_content
            .split_whitespace()
            .map(|f| f.parse::<isize>().context("Failed to parse frequency"))
            .collect::<Result<_>>()?;
        freqs.sort_unstable();

        Ok(Self {
            policy,
            path,
            freqs,
        })
    }

    fn verify_freq(&self, target_freq: isize) -> Result<()> {
        let min_freq = *self.freqs.first().context("No frequencies available")?;
        let max_freq = *self.freqs.last().context("No frequencies available")?;

        if target_freq > max_freq && target_freq < min_freq {
            warn!("CPU Policy{}: target freq out of freq range", self.policy);
        }
        Ok(())
    }

    pub fn write_freq(
        &self,
        target_max_freq: isize,
        target_min_freq: isize,
        files_handler: &mut FilesHandler,
    ) -> Result<()> {
        self.verify_freq(target_max_freq)
            .context("Failed to freq verify")?;
        self.verify_freq(target_min_freq)
            .context("Failed to freq verify")?;

        files_handler.write_with_handler(
            self.path.join("scaling_max_freq"),
            target_max_freq.to_string(),
        )?;
        files_handler.write_with_handler(
            self.path.join("scaling_min_freq"),
            target_min_freq.to_string(),
        )?;
        Ok(())
    }
}
