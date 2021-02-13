use clap::ArgMatches;
use colored::Colorize;
use log::{self, Level, LevelFilter, Log, Metadata, Record};

pub static LOGGER: Logger = Logger;
pub struct Logger;

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub fn init(arg_matches: &ArgMatches) -> anyhow::Result<()> {
	log::set_logger(&LOGGER)?;
	log::set_max_level(
		arg_matches
			.value_of("log-level")
			.map(|s| s.parse().unwrap())
			.unwrap_or(LOG_LEVEL),
	);
	Ok(())
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
				}
				Level::Warn => {
					eprintln!("{}: {}", "WARNING".bright_yellow(), record.args());
				}
				Level::Info => {
					println!("{}", record.args());
				}
				Level::Debug => {
					println!("{}: {}", "DEBUG".bright_cyan(), record.args());
				}
				Level::Trace => {
					println!("{}: {}", "TRACE".bright_cyan(), record.args());
				}
			}
		}
	}

	fn flush(&self) {}
}
