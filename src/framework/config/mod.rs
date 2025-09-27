pub mod data;
mod read;

use std::{
    fs,
    path::Path,
    sync::mpsc::{self, Receiver},
    thread,
};

use anyhow::Result;
use log::error;

use crate::framework::config::{data::ConfigData, read::wait_until_read};

#[derive(Debug)]
pub struct Config {
    rx: Receiver<ConfigData>,
    config: ConfigData,
}

impl Config {
    pub fn new<P>(p: P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let p = p.as_ref();
        let toml_raw = fs::read_to_string(p)?;
        let toml: ConfigData = toml::from_str(&toml_raw)?;
        let (sx, rx) = mpsc::channel();

        {
            let p = p.to_owned();
            thread::Builder::new()
                .name("ConfigWatcher".into())
                .spawn(move || {
                    wait_until_read(p, &sx).unwrap_or_else(|e| {
                        error!("{e:#?}");
                        panic!();
                    });
                })
                .unwrap();
        }

        Ok(Self { rx, config: toml })
    }

    pub fn config(&mut self) -> &mut ConfigData {
        if let Some(s) = self.rx.try_iter().last() {
            self.config = s;
        }

        &mut self.config
    }
}
