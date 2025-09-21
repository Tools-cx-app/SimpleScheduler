use anyhow::Result;

use crate::framework::config::data::ConfigData;

pub struct Looper {
    config: ConfigData,
}

impl Looper {
    pub fn new(c: ConfigData) -> Self {
        Self { config: c }
    }

    pub fn enter_looper(&self) -> Result<()> {
        loop {}
    }
}
