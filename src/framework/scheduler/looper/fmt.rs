use std::fmt::{self};

use super::SimpleSchedulerMode;

impl fmt::Display for SimpleSchedulerMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Powersave => write!(f, "powersave"),
            Self::Balance => write!(f, "balance"),
            Self::Performance => write!(f, "performance"),
        }
    }
}
