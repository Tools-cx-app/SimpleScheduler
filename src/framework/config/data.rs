use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    config: Config,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub general: String,
    pub powersave: HashSet<String>,
    pub bablance: HashSet<String>,
    pub performance: HashSet<String>,
}

