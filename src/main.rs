#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap
)]

mod cpu;
mod files_handler;
mod framework;
mod msic;

use std::io::Write;

use anyhow::Result;
use env_logger::Builder;
use mimalloc::MiMalloc;

use framework::scheduler;

use crate::framework::config::Config;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<()> {
    let mut builder = Builder::new();

    builder.format(|buf, record| {
        let local_time = chrono::Local::now();
        let time_str = local_time.format("%Y-%m-%d %H:%M:%S%.3f").to_string();

        writeln!(
            buf,
            "[{}] [{}] [{}] {}",
            time_str,
            record.level(),
            record.target(),
            record.args()
        )
    });
    builder.filter_level(log::LevelFilter::Info).init();

    let mut config = Config::new("")?;

    scheduler::Scheduler::new().config(config).start_run()
}
