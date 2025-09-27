use std::{fs, path::Path, sync::mpsc::Sender, time::Duration};

use anyhow::Result;
use inotify::{Inotify, WatchMask};
use log::{debug, error, info};

use crate::framework::config::data::ConfigData;

const MAX_RETRY_COUNT: u8 = 20;

pub(super) fn wait_until_modify<P>(path: P, sx: &Sender<ConfigData>) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    loop {
        match read_config_with_retry(path) {
            Ok(s) => sx.send(s).unwrap(),
            Err(e) => {
                error!("Too many retries reading config: {e}");
                panic!();
            }
        }
        wait_until_update(path)?;
    }
}

fn read_config(path: &Path) -> Result<ConfigData> {
    let content = fs::read_to_string(path)?;
    let config = toml::from_str(&content)?;
    Ok(config)
}

fn wait_until_update(path: &Path) -> Result<()> {
    let mut inotify = Inotify::init()?;

    inotify
        .watches()
        .add(path, WatchMask::MODIFY | WatchMask::CLOSE_WRITE)?;

    let mut buffer = [0; 1024];
    inotify.read_events_blocking(&mut buffer)?;

    info!("the config file modifyed, reloaded");

    Ok(())
}

fn read_config_with_retry(path: &Path) -> Result<ConfigData> {
    let mut retry_count = 0;

    loop {
        match read_config(path) {
            Ok(config) => return Ok(config),
            Err(e) => {
                debug!("Failed to read config at {}: {e}", path.display());
                retry_count += 1;
                if retry_count >= MAX_RETRY_COUNT {
                    return Err(e);
                }
                std::thread::sleep(Duration::from_secs(1));
            }
        }
    }
}
