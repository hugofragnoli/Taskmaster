use std::{
	fs::File,
	process::{Child, Command, Stdio},
	thread::sleep,
	time::Duration,
};

use crate::config::parser::parse_config;
use crate::config::structs::{Program, ProgramConfig2, Taskmaster};

extern crate libc;

use libc::{F_VOLPOSMODE, signal};
use libc::{SIGINT, c_int, c_void};
use libc::{exit, sighandler_t};

fn get_handler() -> sighandler_t {
	handler as extern "C" fn(c_int) as *mut c_void as sighandler_t
}

// fn start_program(config: &mut MiniConfig) -> Child {
// 	if config.redirect {
// 		config.stdoutfile = format!("{}_stdout_log.txt", config.name);

// 		let logfile = File::create(config.stdoutfile.clone()).expect("Failed to create logfile");

// 		Command::new(&config.cmd[0])
// 			.stdout(logfile)
// 			.args(&config.cmd[1..])
// 			.spawn()
// 			.expect("failed to start process")
// 	} else {
// 		Command::new(&config.cmd[0])
// 			.args(&config.cmd[1..])
// 			.spawn()
// 			.expect("failed to start process")
// 	}
// }

fn start_sh(program: &mut Program) {
	// ON va mettre un fichier de log par commande ca posera pas de pb dacces DIS MOI CE QUE TEN PENSES BG
	let prog_name = &program.config.0; // "nom du prog bg"
	let args = &program.config.1; // "toute la conf"
	let split_args: Vec<&str> = args.cmd.split_whitespace().collect();
	if let Some(binary) = split_args.get(0) {
		// binary = le nom du binaire quon veut lancer.

		let logfile_name = format!("{}.txt", prog_name);
		let logfile = File::create(&logfile_name).expect("failed to create file");
		let mut child = Command::new(binary)
			.stdout(logfile)
			.args(&split_args[1..])
			.spawn()
			.expect("failed to start");

		println!("🚀 [{}] lancé avec le PID {}", prog_name, child.id());
		program.childs.push(child);
	}
}

fn check_process_status(program: &mut Program) -> bool {
	// peut on check juste avec taskmaster ? en
	// thread ici pour check les status ?
	// checkage d'etat  true = actif false = mort
	// en attendant jfais pas de thread bg
	program.childs.retain_mut(|child| match child.try_wait() {
		Ok(None) => true,
		Ok(Some(_)) => false,
		Err(_) => false,
	});
	program.childs.is_empty();
	return false;
}

pub fn handle_commands_sh(line: &str, taskmaster: &mut Taskmaster) {
	// println!("ENCOURSMAELMENVEUXPASSSS");
	let splitted: Vec<&str> = line.split_whitespace().collect();
	match &splitted[..] {
		["status"] => {
			println!("Printing status...");
			//println!("{}", taskmaster.status) //TODO
		}
		["start", follow_starts @ ..] => {
			for follow_start in follow_starts {
				let mut tmp = follow_start.to_string();
				// on essaie de trouver larg dans la list de prog. .0
				if let Some(p) = taskmaster.programs.iter_mut().find(|p| p.config.0 == tmp) {
					// PEUT ETRE faut quon appelle le thread monitor pour check letat du process.
					if check_process_status(p) {
						println!("Error : Program '{}' already running.", follow_start);
						println!("Please enter a program name currently off.");
					} else {
						println!("Launching : {}", follow_start);
						start_sh(p);
					}
				} else {
					println!("Error : Prog '{}' has not been found on the config.yaml file.", follow_start);
				}
			}
		}
		// ["restart", follow_starts @ ..] => {

		// }
		_ => {
			println!("Error : Invalid command or missing arguments : {}", line);
		}
	}
}

