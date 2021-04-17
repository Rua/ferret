use clap::ArgMatches;
use colored::Colorize;
use crossbeam_channel::Sender;
use log::{self, Level, LevelFilter, Log, Metadata, Record};

#[derive(Clone, Debug)]
struct Logger {
	sender: Sender<String>,
}

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;

#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

pub fn init(arg_matches: &ArgMatches, sender: Sender<String>) -> anyhow::Result<()> {
	log::set_boxed_logger(Box::new(Logger { sender }))?;
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
					self.sender.send(format!("ERROR: {}\n", record.args())).ok();
				}
				Level::Warn => {
					eprintln!("{}: {}", "WARNING".bright_yellow(), record.args());
					self.sender
						.send(format!("WARNING: {}\n", record.args()))
						.ok();
				}
				Level::Info => {
					println!("{}", record.args());
					self.sender
						.send(format!("{}\n", record.args().to_string()))
						.ok();
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
