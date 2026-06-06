use std::error;
use std::io::Write;
use env_logger::Target;
use log::LevelFilter;

pub fn setup_logger(debug: bool) -> Result<(), Box<dyn error::Error>> {
    env_logger::Builder::new()
        .filter_level(
        if debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Error
        })
        .target(Target::Stdout)
        .format(|formatter, record| {
            writeln!(
                formatter,
                "[{}] {} {} {}",
                record.module_path_static().unwrap_or(""),
                record.file().unwrap_or(""),
                record.level(),
                record.args()
            )
        })
        .init();

    Ok(())
}
