#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Level {
	Debug = 0,
	Info,
	Warning,
	Error,
	Critical,
}

const SECS_PER_DAY: u64 = 86400;

const GRAY: &str = "\x1b[90m";

use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct Logger {
	level: Level,
}

impl Logger {
	pub fn new() -> Logger {
		Self { level: Level::Debug }
	}

	pub fn log(&self, msg_level: Level, msg: &str) {
		if msg_level < self.level {
			return;
		}

		let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Erreur de temps").as_secs();

		let secs_in_day = now % 86400;
		let hours = (secs_in_day / 3600 + 1) % 24;
		let minutes = (secs_in_day / 60) % 60;
		let seconds = secs_in_day % 60;

		let color = match msg_level {
			Level::Debug => "\x1b[34m",
			Level::Info => "\x1b[32m",
			Level::Warning => "\x1b[33m",
			Level::Error => "\x1b[31m",
			Level::Critical => "\x1b[1;31m",
		};

		let reset = "\x1b[0m";

		println!(
			"{}[{:02}:{:02}:{:02}]{} {}[{:?}]{} - {}",
			GRAY, hours, minutes, seconds, reset, color, msg_level, reset, msg
		);
	}

	pub fn debug(&self, msg: &str) {
		self.log(Level::Debug, msg);
	}
	pub fn info(&self, msg: &str) {
		self.log(Level::Info, msg);
	}
	pub fn warning(&self, msg: &str) {
		self.log(Level::Warning, msg);
	}
	pub fn error(&self, msg: &str) {
		self.log(Level::Error, msg);
	}
	pub fn critical(&self, msg: &str) {
		self.log(Level::Critical, msg);
	}
}
#[macro_export]
macro_rules! debug {
	($message:expr) => {
		let logger = logger::Logger::new();
		logger.debug($message);
	};
}
