use log::{self, Level, Log, Metadata, Record, SetLoggerError};
use colored::Colorize;

pub static LOGGER: Logger = Logger;


pub struct Logger;

impl Logger {
	pub fn init() -> Result<(), SetLoggerError> {
		log::set_logger(&LOGGER)?;
		log::set_max_level(log::STATIC_MAX_LEVEL);
		Ok(())
	}
}

impl Log for Logger {
	fn enabled(&self, metadata: &Metadata<'_>) -> bool {
		metadata.level() <= log::STATIC_MAX_LEVEL
	}

	fn log(&self, record: &Record<'_>) {
		if self.enabled(record.metadata()) {
			match record.level() {
				Level::Error => {
					eprintln!("{}: {}", "ERROR".bright_red(), record.args());
				},
				Level::Warn => {
					eprintln!("{}: {}", "WARNING".bright_yellow(), record.args());
				},
				Level::Info => {
					println!("{}: {}", "INFO", record.args());
				},
				Level::Debug => {
					println!("{}: {}", "DEBUG".bright_cyan(), record.args());
				},
				Level::Trace => {
					println!("{}", record.args());
				},
			}
		}
	}

	fn flush(&self) { }
}
