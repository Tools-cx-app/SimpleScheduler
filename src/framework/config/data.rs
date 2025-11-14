use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigData {
    pub config: Config,
    pub cpu: Cpu,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub general: String,
    pub powersave: HashSet<String>,
    pub balance: HashSet<String>,
    pub performance: HashSet<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cpu {
    pub governors: Governors,
    pub powersave: Freqs,
    pub balance: Freqs,
    pub performance: Freqs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Freqs {
    pub small: (isize, isize),
    pub middle: (isize, isize),
    pub big: (isize, isize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Governors {
    pub general: String,
    pub powersave: String,
    pub balance: String,
    pub performance: String,
}
