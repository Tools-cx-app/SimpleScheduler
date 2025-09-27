use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Missing {0} when binding looper")]
    SchedulerMissing(&'static str),
    #[error("Failed to config parse {0}")]
    ConfigParse(&'static str),
}
