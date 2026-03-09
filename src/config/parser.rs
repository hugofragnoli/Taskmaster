use std::{env, fs::File, process::exit};

use crate::error;

use serde_yaml::from_reader;
use std::time::Instant;

use crate::config::structs::{Program, ProgramsConfig, Taskmaster};

fn parse_config2(f: File) -> Result<ProgramsConfig, Box<dyn std::error::Error>> {
	let d: ProgramsConfig = from_reader(f)?;
	Ok(d)
}

/// Parse config file
/// 1. Read config file and serialize it
/// 2. Check incompatible value in a program config
/// 3. return TaskMaster struct
pub fn parse_config() -> Taskmaster {
	let args: Vec<String> = env::args().collect();

	let path: String;

	if args.len() > 2 {
		error!("Too many arguments");
		exit(1);
	} else if args.len() == 2 {
		path = args[1].clone();
	} else {
		path = String::from("configmael.yaml");
	}

	let f = std::fs::File::open(&path);
	match f {
		Ok(file) => {
			let config = parse_config2(file);
			match config {
				Ok(conf) => {
					let mut tm: Taskmaster = Taskmaster { programs: vec![] };

					for p in conf.programs.iter().enumerate().clone() {
						tm.programs.push(Program {
							config: (p.1.0.clone(), p.1.1.clone()),
							childs: Vec::new(),
							retry_count: 0,
							last_launch_time: Instant::now(),
						});
					}
					tm
				}
				Err(e) => {
					error!(format!("Unable to  parse config : {}", e));
					exit(1);
				}
			}
		}
		Err(e) => {
			error!(format!("Unable to open file {} : {}", path, e));
			exit(1);
		}
	}
}
