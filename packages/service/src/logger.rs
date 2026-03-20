use std::path::PathBuf;

use log::LevelFilter;
use log4rs::{
    Config,
    append::{
        console::ConsoleAppender,
        rolling_file::{
            RollingFileAppender,
            policy::compound::{
                CompoundPolicy, roll::fixed_window::FixedWindowRoller,
                trigger::size::SizeTrigger,
            },
        },
    },
    config::{Appender, Root},
    encode::json::JsonEncoder,
};

use crate::constant::env::ENV_NAME_LOG_DIR;

const LOG_FILE_NAME: &str = "service.log";
const LOG_ARCHIVE_PATTERN: &str = "service.{}.log.gz";
const LOG_FILE_MAX_SIZE: u64 = 10 * 1024 * 1024; // 10 MB
const LOG_ARCHIVE_COUNT: u32 = 5;

pub fn init_logger() -> anyhow::Result<()> {
    let stdout = ConsoleAppender::builder().build();

    let log_dir: PathBuf = std::env::var(ENV_NAME_LOG_DIR)?.into();

    let log_path = log_dir.join(LOG_FILE_NAME);
    let archive_pattern =
        log_dir.join(LOG_ARCHIVE_PATTERN).display().to_string();

    let roller = FixedWindowRoller::builder()
        .build(&archive_pattern, LOG_ARCHIVE_COUNT)?;
    let trigger = SizeTrigger::new(LOG_FILE_MAX_SIZE);
    let policy = CompoundPolicy::new(Box::new(trigger), Box::new(roller));

    let file_appender = RollingFileAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build(log_path, Box::new(policy))?;

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(LevelFilter::Info),
        )?;

    let _ = log4rs::init_config(config)?;
    Ok(())
}
