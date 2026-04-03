#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Level {
	Debug = 0,
	Info,
	Error,
	Critical,
}

const GRAY: &str = "\x1b[90m";

use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct Logger {
	level: Level,
}

impl Logger {
	pub fn new() -> Logger {
		Self {
			level: Level::Debug,
		}
	}

	pub fn log(&self, msg_level: Level, msg: &str) {
		if msg_level < self.level {
			return;
		}

		let now = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.expect("Time error.")
			.as_secs();

		let secs_in_day = now % 86400;
		let hours = (secs_in_day / 3600 + 1) % 24;
		let minutes = (secs_in_day / 60) % 60;
		let seconds = secs_in_day % 60;

		let color = match msg_level {
			Level::Debug => "\x1b[34m",
			Level::Info => "\x1b[32m",
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
	pub fn error(&self, msg: &str) {
		self.log(Level::Error, msg);
	}
	pub fn critical(&self, msg: &str) {
		self.log(Level::Critical, msg);
	}
}

pub static INSTANCE: OnceLock<Mutex<Logger>> = OnceLock::new();

pub fn get_logger() -> &'static Mutex<Logger> {
	INSTANCE.get_or_init(|| Mutex::new(Logger::new()))
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr, $($arg:tt)*) => {
        {
            if let Ok(logger) = $crate::logger::get_logger().lock() {
                logger.debug(&format!($fmt, $($arg)*));
            }
        }
    };
    ($message:expr) => {
        {
            if let Ok(logger) = $crate::logger::get_logger().lock() {
                logger.debug($message.as_ref());
            }
        }
    };
}

#[macro_export]
macro_rules! info {
    ($fmt:expr, $($arg:tt)*) => {
        {
            if let Ok(logger) = $crate::logger::get_logger().lock() {
                logger.info(&format!($fmt, $($arg)*));
            }
        }
    };
    ($message:expr) => {
        {
            if let Ok(logger) = $crate::logger::get_logger().lock() {
                logger.info($message.as_ref());
            }
        }
    };
}

#[macro_export]
macro_rules! error {
    ($fmt:expr, $($arg:tt)*) => {
        {
            if let Ok(logger) = $crate::logger::get_logger().lock() {
                logger.error(&format!($fmt, $($arg)*));
            }
        }
    };
    ($message:expr) => {
        {
            if let Ok(logger) = $crate::logger::get_logger().lock() {
                logger.error($message.as_ref());
            }
        }
    };
}

#[macro_export]
macro_rules! critical {
    ($fmt:expr, $($arg:tt)*) => {
        {
            if let Ok(logger) = $crate::logger::get_logger().lock() {
                logger.critical(&format!($fmt, $($arg)*));
            }
        }
    };
    ($message:expr) => {
        {
            if let Ok(logger) = $crate::logger::get_logger().lock() {
                logger.critical($message.as_ref());
            }
        }
    };
}
