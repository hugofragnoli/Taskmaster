use std::{
	fs::File,
	process::{Child, Command, Stdio},
	thread::sleep,
	time::Duration,
};

use crate::ProgramConfig;

extern crate libc;

use libc::signal;
use libc::{SIGINT, c_int, c_void};
use libc::{exit, sighandler_t};

// redirige stdio du premier process dans stdin du second
fn pipe_process() {
	let ls = Command::new("ls")
		.stdout(Stdio::piped())
		.spawn()
		.expect("Failed to execute ls");

	let ls_stdout = ls.stdout.expect("failed to pipe stdout of ls process");

	let cat = Command::new("cat")
		.stdin(Stdio::from(ls_stdout))
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute cat");

	let cat_stdout = cat
		.wait_with_output()
		.expect("failed to pipe stdout of cat process");

	println!("{}", String::from_utf8_lossy(cat_stdout.stdout.as_slice()));
}

// simule un programme long avec redirection d'output dans un fichier de log.
fn redirect_to_file() {
	let logfile = File::create("logfile.txt").expect("failed to open file");
	let mut ping = Command::new("ping")
		.stdout(logfile)
		.args(&["google.com"])
		.spawn()
		.expect("failed to start ping");

	sleep(Duration::from_mins(2)); // sleep 2 minutes
	ping.kill().expect("Failed to kill ping");
}

extern "C" fn handler(_: c_int) {
	println!("aaaaaaaaaaaa");
	// exit(0);
}

fn get_handler() -> sighandler_t {
	handler as extern "C" fn(c_int) as *mut c_void as sighandler_t
}

pub struct MiniConfig {
	pub name: String,
	pub cmd: Vec<String>,
	pub restart_always: bool,
	pub redirect: bool,
	pub stdoutfile: String,
	pub finished: bool,
}

fn start_program(config: &mut MiniConfig) -> Child {
	if config.redirect {
		config.stdoutfile = format!("{}_stdout_log.txt", config.name);

		let logfile = File::create(config.stdoutfile.clone()).expect("Failed to create logfile");

		Command::new(&config.cmd[0])
			.stdout(logfile)
			.args(&config.cmd[1..])
			.spawn()
			.expect("failed to start process")
	} else {
		Command::new(&config.cmd[0])
			.args(&config.cmd[1..])
			.spawn()
			.expect("failed to start process")
	}
}

pub fn exec_and_monitor() {
	// pipe_process();

	// redirect_to_file();

	// unsafe {
	// 	signal(SIGINT, get_handler());
	// }
	//
	// loop {
	// 	println!("in loop");
	// 	std::thread::sleep(Duration::from_millis(100));
	// }

	let mut programs = [
		MiniConfig {
			name: "ping1".to_string(),
			cmd: vec![
				"ping".to_string(),
				"google.com".to_string(),
				"-c".to_string(),
				"10".to_string(),
			],
			restart_always: true,
			redirect: true,
			stdoutfile: "".to_string(),
			finished: false,
		},
		MiniConfig {
			name: "ping2".to_string(),
			cmd: vec!["ping".to_string(), "pornhub.com".to_string()],
			restart_always: false,
			redirect: false,
			stdoutfile: "".to_string(),
			finished: false,
		},
	];

	let mut childs: Vec<Child> = vec![];

	for program in &mut programs {
		println!("Starting {}", program.name);
		childs.push(start_program(program));
	}

	loop {
		let mut count_finished = 0;
		for (i, child) in childs.iter_mut().enumerate() {
			match child.try_wait() {
				Ok(Some(status)) => {
					if programs[i].finished {
						continue;
					}
					println!("{} exited with: {status}", programs[i].name);
					programs[i].finished = true;
					count_finished += 1;
					// if programs[i].restart_always {
					// 	childs[i] = start_program(&mut programs[i]);
					// }
				}
				Ok(None) => {
					// println!("program {} running", programs[i].name);
				}
				Err(e) => println!("error attempting to wait: {e} {}", programs[i].name),
			}
		}
		if count_finished == programs.len() {
			break;
		}
	}
}
