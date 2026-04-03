use std::env;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Stdio;
use std::{fs::OpenOptions, process::Command};

use std::time::Instant;

use libc::umask;

use crate::config::structs::{_Restart, Program, Taskmaster};
use crate::{error, info};

pub fn print_status(taskmaster: &Taskmaster, target_prog: Option<&str>) {
	for program in &taskmaster.programs {
		let prog_name = &program.config.0;

		if let Some(target) = target_prog
			&& prog_name != target
		{
			continue;
		}

		let is_running = !program.childs.is_empty();
		let pids: Vec<u32> = program.childs.iter().map(|c| c.id()).collect();
		if is_running {
			info!("[STATUS] {} is alive (PIDs: {:?})", prog_name, pids);
		} else {
			info!("[STATUS] {} is off", prog_name);
		}
	}
}

pub fn start_prog(program: &mut Program, print_message: bool, num_to_start: usize) {
	program.is_stopped_manually = false;
	let prog_name = &program.config.0;
	let args = &program.config.1;
	let split_args: Vec<&str> = args.cmd.split_whitespace().collect();

	program.last_launch_time = Instant::now();

	if let Some(binary) = split_args.first() {
		let bin_path = Path::new(binary);

		let executable = if !binary.contains('/') {
			bin_path.to_path_buf()
		} else if bin_path.is_absolute() {
			bin_path.to_path_buf()
		} else {
			env::current_dir()
				.expect("Error : Unable to read current directory.")
				.join(bin_path)
		};

		let mut cmd = Command::new(executable);

		cmd.args(&split_args[1..]);

		if let Some(ref dir) = args.working_dir {
			cmd.current_dir(dir);
		}

		if let Some(ref envs) = args.env_to_set {
			cmd.envs(envs);
		}

		if let Some(mask_val) = args.umask {
			unsafe {
				cmd.pre_exec(move || {
					umask(mask_val.into());
					Ok(())
				});
			}
		}

		if let Some(redirect) = &args.redirect {
			let stdout = redirect.stdout.clone();
			let stderr = redirect.stderr.clone();

			let logfilestdout = OpenOptions::new()
				.create(true)
				.append(true)
				.open(&stdout)
				.expect("failed to open log file for stdout");

			let logfilestderr = OpenOptions::new()
				.create(true)
				.append(true)
				.open(&stderr)
				.expect("failed to open log file for stderr");

			cmd.stdout(logfilestdout);
			cmd.stderr(logfilestderr);
		} else {
			cmd.stdout(Stdio::null());
			cmd.stderr(Stdio::null());
		}

		for _ in 0..num_to_start {
			match cmd.spawn() {
				Ok(child) => {
					if print_message {
						info!("Program [{}] launch with PID {}", prog_name, child.id());
					}
					program.childs.push(child);
				}
				Err(e) => {
					if print_message {
						error!("Error during program [{}] launch : {}", prog_name, e);
					}
				}
			}
		}
	}
}

pub fn stop_prog(program: &mut Program) {
	program.is_stopped_manually = true;
	for child in &mut program.childs {
		let _result = child.kill();
		child.wait().expect("Unable to kill process");
	}
	program.childs.clear();
}

fn should_relaunch(program: &mut Program) -> bool {
	let config = &program.config.1;

	if program.is_stopped_manually {
		return false;
	}

	match config.restart_policy {
		_Restart::Never => return false,
		_Restart::UnexpectedExits => {
			if !program.unexpected_error_code {
				return false;
			}
		}
		_Restart::Always => (),
	}

	if program.retry_count >= config.max_relauch_retry {
		return false;
	}

	let wait_time = config.minimum_runtime.unwrap_or(1);
	if program.last_launch_time.elapsed().as_secs() < wait_time {
		return false;
	}

	if program.unexpected_error_code {
		program.unexpected_error_code = false;
	}

	true
}

pub fn check_process_status(taskmaster: &mut Taskmaster) {
	for program in &mut taskmaster.programs {
		let _prog_name = &program.config.0.clone();
		let config = &program.config.1;
		program.childs.retain_mut(|child| match child.try_wait() {
			Ok(Some(status)) => {
				if program.is_stopped_manually {
					return false;
				}

				let exit_code = status.code();

				if let Some(code) = exit_code {
					if let Some(errors_code) = &program.config.1.expected_error_codes {
						if !errors_code.contains(&(code as u32)) {
							program.unexpected_error_code = true;
						}
					} else if code != 0 {
						program.unexpected_error_code = true;
					}
				} else {
					program.unexpected_error_code = true;
				}

				let min_run = program.config.1.minimum_runtime.unwrap_or(1);
				if program.last_launch_time.elapsed().as_secs() < min_run {
					program.retry_count += 1;
				} else {
					program.retry_count = 0;
				}
				false
			}
			Ok(None) => true,
			Err(_) => false,
		});

		let current_len = program.childs.len();
		let target_len = config.num_processes as usize;

		if current_len < target_len && should_relaunch(program) {
			let missing = target_len - current_len;
			start_prog(program, false, missing);
		}
	}
}
