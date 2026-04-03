use std::{env, fs::File};

use crate::errors::TaskmasterError;

use serde_yaml::from_reader;
use std::time::Instant;

use crate::config::structs::{_Signalstopper, Program, ProgramsConfig, Taskmaster};

fn is_config_valid(config: &Taskmaster) -> Result<(), TaskmasterError> {
	for program in &config.programs {
		let Some(sig) = &program.config.1.stop_signal else {
			continue;
		};
		if *sig == _Signalstopper::SIGHUP {
			return Err(TaskmasterError::InvalidParam(format!(
				"Invalid signal for program {}. SIGHUP is reserved for configuration reload.",
				program.config.0
			)));
		}
	}
	Ok(())
}

fn parse_config_file(f: File) -> Result<ProgramsConfig, TaskmasterError> {
	let d: ProgramsConfig = from_reader(f)?;
	Ok(d)
}

/// Parse config file
/// 1. Read config file and serialize it
/// 2. Check incompatible value in a program config
/// 3. return TaskMaster struct
pub fn parse_config() -> Result<Taskmaster, TaskmasterError> {
	let args: Vec<String> = env::args().collect();

	let path: String = match args.len() {
		1 => String::from("config.yaml"),
		2 => args[1].clone(),
		_ => {
			return Err(TaskmasterError::Argument(
				"Too many arguments provided".to_string(),
			));
		}
	};

	let f = std::fs::File::open(&path)?;
	let config = parse_config_file(f)?;

	let mut tm: Taskmaster = Taskmaster {
		programs: Vec::with_capacity(config.programs.len()),
	};

	// initialize programs vector
	for (name, prog_config) in config.programs.into_iter() {
		tm.programs.push(Program {
			config: (name, prog_config),
			childs: Vec::new(),
			retry_count: 0,
			last_launch_time: Instant::now(),
			unexpected_error_code: false,
			is_stopped_manually: false,
		});
	}

	is_config_valid(&tm)?;
	Ok(tm)
}
