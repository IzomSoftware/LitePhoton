use std::error;

use log::LevelFilter;
use log4rs::Config;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn setup_logger(debug: bool) -> Result<(), Box<dyn error::Error>> {
    let level = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Error
    };

    let config = Config::builder()
            .appender(
                Appender::builder().build(
                    "stdout",
                    Box::new(
                        ConsoleAppender::builder()
                            .encoder(Box::new(PatternEncoder::new("[LitePhoton] {l} {m}\n")))
                            .build(),
                    ),
                ),
            )
            .build(Root::builder().appender("stdout").build(level))?;

    log4rs::init_config(
        config
    )?;

    Ok(())
}
