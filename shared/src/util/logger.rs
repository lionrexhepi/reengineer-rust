use std::path::Path;
use std::time::SystemTime;

use chrono::{ TimeZone, DateTime, Utc, Local };
use log::info;
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;

use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::Policy;

use log4rs::append::rolling_file::policy::compound::roll::delete;
use log4rs::config::Appender;
use log4rs::config::Logger;
use log4rs::config::Root;
use log4rs::init_config;
use once_cell::sync::Lazy;

use super::Boxable;

static IS_INIT: Lazy<bool> = Lazy::new(|| false);

#[derive(Debug)]
struct OnStartupPolicy {}

impl Policy for OnStartupPolicy {
    fn process(&self, log: &mut log4rs::append::rolling_file::LogFile) -> anyhow::Result<()> {
        if let None = Lazy::get(&IS_INIT) {
            let _ = *IS_INIT;

            let meta = std::fs::metadata(log.path());

            let timestamp: DateTime<Local> = (
                match meta {
                    Ok(data) =>
                        match data.created() {
                            Ok(created) => Some(created),
                            Err(_) =>
                                match data.accessed() {
                                    Ok(accessed) => Some(accessed),
                                    Err(_) => None,
                                }
                        }
                    Err(_) => None,
                }
            )
                .unwrap_or(SystemTime::now())
                .into();

            let filename = log
                .path()
                .parent()
                .unwrap_or(Path::new("/"))
                .join(Path::new(&timestamp.format("%d:%m:%Y-%T.log").to_string()));

            std::fs::copy(log.path(), filename)?;
            std::fs::remove_file(log.path())?;
            std::fs::write(log.path(), "")?;

            log.roll();

            Ok(())
        } else {
            Ok(())
        }
    }
}

pub fn default_config() -> log4rs::Handle {
    let stdout = ConsoleAppender::builder().build();

    let logfile = RollingFileAppender::builder()
        .build("logs/latest.log", (OnStartupPolicy {}).boxed())
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", stdout.boxed()))
        .appender(Appender::builder().build("file", logfile.boxed()))
        .logger(
            Logger::builder()
                .appender("stdout")
                .appender("file")
                .build("main", log::LevelFilter::max())
        )
        .build(Root::builder().appender("stdout").appender("file").build(log::LevelFilter::max()));

    let handle = init_config(config.unwrap()).unwrap();

    info!("New instance started, latest.log was copied here"); //Will be written to the old latest.log before it is moved

    handle
}