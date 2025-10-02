use std::{fs, path::PathBuf};

use anyhow::Result;
use log::warn;

use crate::cpu::freqs::CpuFreqs;

pub fn get_freqs(freqs: &mut CpuFreqs, conut: isize) -> isize {
    freqs.freqs[conut as usize]
}

pub fn read_policy() -> Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for entry in fs::read_dir("/sys/devices/system/cpu/cpufreq")? {
        let path = match entry {
            Ok(entry) => entry.path(),
            Err(e) => {
                warn!("Failed to read entry: {e:?}");
                continue;
            }
        };

        if !path.is_dir() {
            continue;
        }

        let Some(filename) = path.file_name().and_then(|s| s.to_str()) else {
            continue;
        };

        if !filename.starts_with("policy") {
            continue;
        }

        paths.push(path);
    }
    Ok(paths)
}
