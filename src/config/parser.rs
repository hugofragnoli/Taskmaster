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
		_ => return Err(TaskmasterError::Argument("Too many arguments provided".to_string())),
	};

	let f = std::fs::File::open(&path)?;
	let config = parse_config_file(f)?;

	let mut tm: Taskmaster = Taskmaster {
		programs: Vec::with_capacity(config.programs.len()),
		config_file: path.clone(),
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

// pub fn parse_config() -> Result<Taskmaster, String> {
// 	let args: Vec<String> = env::args().collect();
//
// 	let path: String;
//
// 	if args.len() > 2 {
// 		error!("Too many arguments");
// 		exit(1);
// 	} else if args.len() == 2 {
// 		path = args[1].clone();
// 	} else {
// 		path = String::from("config.yaml");
// 	}
//
// 	let f = std::fs::File::open(&path);
// 	match f {
// 		Ok(file) => {
// 			let config = parse_config2(file);
// 			match config {
// 				Ok(conf) => {
// 					let mut tm: Taskmaster = Taskmaster { programs: vec![], config_file: path.clone() };
//
// 					for p in conf.programs.iter().enumerate().clone() {
// 						tm.programs.push(Program {
// 							config: (p.1.0.clone(), p.1.1.clone()),
// 							childs: Vec::new(),
// 							retry_count: 0,
// 							last_launch_time: Instant::now(),
// 						});
// 					}
//
//                     if is_config_valid(&tm) {
//                         return Ok(tm);
//                     }
//                     Err(String::from("Invalid config"))
// 				}
// 				Err(e) => {
// 					error!(format!("Unable to  parse config : {}", e));
//                     Err(String::from(format!("Unable to  parse config : {}", e)))
// 				}
// 			}
// 		}
// 		Err(e) => {
// 			error!(format!("Unable to open file {} : {}", path, e));
//                     Err(String::from(format!("Unable to open file {} : {}", path, e)))
// 		}
// 	}
// }
