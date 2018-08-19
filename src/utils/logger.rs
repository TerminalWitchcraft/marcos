use log;
use fern;

/// Initialize the looger.
/// Currently lacks flexibility to specify log levels, output streams, etc
///
/// TODO: Log level as input, output stream as input, default log file format
pub fn init() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "==> {} [{}] -> {}",
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("./log/output.log")?)
        .apply()?;
    Ok(())
}
