use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigData {
    pub config: Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: String,
    pub powersave: HashSet<String>,
    pub bablance: HashSet<String>,
    pub performance: HashSet<String>,
}
