use fern;
use log;

use error::*;

/// Initialize the looger.
/// Currently lacks flexibility to specify log levels, output streams, etc
///
/// TODO: Log level as input, output stream as input, default log file format
pub fn init(file_name: Option<&str>, log_level: Option<&str>) -> Result<()> {
    let logger = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "==> {} [{}] -> {}",
                record.target(),
                record.level(),
                message
            ))
        }).level_for("cursive", log::LevelFilter::Warn)
        .level(match log_level {
            Some(c) => match c {
                "debug" => log::LevelFilter::Debug,
                "info" => log::LevelFilter::Info,
                "error" => log::LevelFilter::Error,
                _ => log::LevelFilter::Off,
            },
            None => log::LevelFilter::Off,
        });
    //.chain(fern::log_file(file_name).expect("Incorrect Log file format"))
    //.apply()?;
    if let Some(c) = file_name {
        logger.chain(fern::log_file(c)?).apply()?;
    } else {
        logger.apply()?;
    }
    Ok(())
}
