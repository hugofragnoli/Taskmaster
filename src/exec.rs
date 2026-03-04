use std::{
	fs::File,
	process::Command,
};

use crate::config::structs::{Program, Taskmaster};

// extern crate libc;

// use libc::{F_VOLPOSMODE, signal};
// use libc::{SIGINT, c_int, c_void};
// use libc::{exit, sighandler_t};

// fn get_handler() -> sighandler_t {
// 	handler as extern "C" fn(c_int) as *mut c_void as sighandler_t
// }

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

pub fn start_prog(program: &mut Program) {
	// ON va mettre un fichier de log par commande ca posera pas de pb dacces DIS MOI CE QUE TEN PENSES BG
	let prog_name = &program.config.0; // "nom du prog bg"
	let args = &program.config.1; // "toute la conf"
	let split_args: Vec<&str> = args.cmd.split_whitespace().collect();
	if let Some(binary) = split_args.get(0) {
		// binary = le nom du binaire quon veut lancer.

		let logfile_name = format!("{}.txt", prog_name);
		let logfile = File::create(&logfile_name).expect("failed to create file");
		match Command::new(binary)
            .stdout(logfile)
            .args(&split_args[1..])
            .spawn()
		{
			Ok(child) => {
				println!("Program [{}] launch with PID {}", prog_name, child.id());
				program.childs.push(child);
			}
			Err(e) => println!("Error during program [{}] launch : {}", prog_name, e),
		}
	}
}

pub fn check_process_status(taskmaster: &mut Taskmaster) {
	//ADAPTATION POUR COLLER A la struct des threads en cours
	for program in &mut taskmaster.programs {
        let prog_name = &program.config.0.clone();
		program.childs.retain_mut(|child| match child.try_wait() {
			Ok(Some(status)) => {
                println!("[{}] has stopped with status: {}", prog_name, status);
                // TODO plus tard: C'est ici qu'on gérera le "restart_always"
                false // Le process est mort, on le retire de la liste des vivantss
            }
            Ok(None) => {
                true // Le process tourne encore, on le garde
            }
            Err(e) => {
                println!("Error while checking status of [{}]: {}", prog_name, e); // check qui fail.
                false
            }
		});
	}
}