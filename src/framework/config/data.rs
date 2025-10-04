use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigData {
    pub config: Config,
    pub freqs: Freqs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: String,
    pub powersave: HashSet<String>,
    pub balance: HashSet<String>,
    pub performance: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Freqs {
    pub general: (isize, isize),
    pub powersave: (isize, isize),
    pub balance: (isize, isize),
    pub performance: (isize, isize),
}
