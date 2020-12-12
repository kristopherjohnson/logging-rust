use chrono::format::{DelayedFormat, StrftimeItems};
use chrono::{DateTime, Local};

use log::{debug, error, info, trace, warn, LevelFilter, Log, SetLoggerError};

use std::thread;
use std::time::{Duration, SystemTime};

/// Custom logger type.
struct CustomLog;

impl CustomLog {
    /// Format current system time.
    fn format_timestamp(&self) -> DelayedFormat<StrftimeItems> {
        let dt: DateTime<Local> = SystemTime::now().into();
        dt.format("%Y-%m-%d %T.%3f")
    }
}

/// Implementation of the `Log` trait.
impl Log for CustomLog {
    /// Format and print log message to stderr.
    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            eprintln!(
                "{} {:>5} [{}][{}:{}:{}] {}",
                self.format_timestamp(),
                record.metadata().level(),
                record.metadata().target(),
                record
                    .module_path()
                    .or_else(|| Some("<no module>"))
                    .unwrap(),
                record.file().or_else(|| Some("<no line>")).unwrap(),
                record.line().or_else(|| Some(0)).unwrap(),
                record.args(),
            )
        }
    }

    /// We enable all log messages.
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    /// `flush` does nothing.
    fn flush(&self) {}
}

static LOGGER: CustomLog = CustomLog;

/// Initialize the custom logger.
pub fn init_logging() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Trace))
}

/// Generate a bunch of log messages to see how they look.
pub fn test_logging() {
    const DELAY: Duration = Duration::from_millis(5);

    for _ in 1..4 {
        error!(target: "test1", "This is an error message");
        thread::sleep(DELAY);

        warn!(target: "test2", "This is a warning message");
        thread::sleep(DELAY);

        info!(target: "test3", "This is an informational message");
        thread::sleep(DELAY);

        debug!(target: "test4", "This is a debug message");
        thread::sleep(DELAY);

        trace!(target: "test5", "This is a trace message");
        thread::sleep(DELAY);

        info!("This informational message has no target specified");
        thread::sleep(DELAY);
    }
}
