use std::{
	process::Command,
    fs::OpenOptions,
};

use crate::config::structs::{Program, Taskmaster, _Restart};

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

//ici on met une option sur le target prog pour differencier le print de tous les status ou dun seul prog. 
// si une target prog est fournie on compare pour print que ca.
pub fn print_status(taskmaster: &Taskmaster, target_prog: Option<&str>) {
	for program in &taskmaster.programs {
        let prog_name = &program.config.0;

        if let Some(target) = target_prog {
            if prog_name != target {
                continue;
            }
        }

		let is_running = !program.childs.is_empty();
		let pids: Vec<u32> = program.childs.iter().map(|c| c.id()).collect();
		if is_running {
            println!("[STATUS] {} est EN COURS (PIDs: {:?})", prog_name, pids);
        } else {
            println!("[STATUS] {} est ARRÊTÉ", prog_name);
        }
	}
}

pub fn start_prog(program: &mut Program) {
	// ON va mettre un fichier de log par commande ca posera pas de pb dacces DIS MOI CE QUE TEN PENSES BG
	let prog_name = &program.config.0; // "nom du prog bg"
	let args = &program.config.1; // "toute la conf"
	let split_args: Vec<&str> = args.cmd.split_whitespace().collect();
	if let Some(binary) = split_args.first() {
        let mut cmd = Command::new(binary);
        cmd.args(&split_args[1..]);
    
		// binary = le nom du binaire quon veut lancer.

        if let Some(ref dir) = args.working_dir {
            cmd.current_dir(dir);
        }

        if let Some(ref envs) = args.env_to_set {
            cmd.envs(envs);
        }
		let logfile_name = format!("{}.txt", prog_name);
        //openoptions permet de lui dire dappend plutot que decrire par dessus si on lance 4 proc en mm temps par ex
		let logfile = OpenOptions::new()
            .create(true)
            .append(true) // <--- Crucial pour ne pas effacer les logs des autres instances
            .open(&logfile_name)
            .expect("failed to open log file");
        cmd.stdout(logfile);
		match cmd.spawn() 
		    {
			Ok(child) => {
				println!("Program [{}] launch with PID {}", prog_name, child.id());
				program.childs.push(child);
			}
			Err(e) => println!("Error during program [{}] launch : {}", prog_name, e),
		    }
        }
	}

pub fn stop_prog(program: &mut Program) {
    for child in &mut program.childs {
        println!("trying to kill process");
        let result = child.kill();
        // wait necessaire pour tuer le process jsp pourquoi ??
        // kill seul envoi le signal mais si on wait pas ca marche pas
        child.wait().expect("Impossible de tuer le processus");
        println!("{:?}", result);
    }
    program.childs.clear();
}

pub fn check_process_status(taskmaster: &mut Taskmaster) {


	for program in &mut taskmaster.programs {
        let prog_name = &program.config.0.clone();
        let config = &program.config.1;
		program.childs.retain_mut(|child| match child.try_wait() {
			Ok(Some(status)) => {
                let exit_code = status.code();
                println!("[{}] has stopped with status: {}", prog_name, status);

                let should_restart = match config.restart_policy {
                    _Restart::Always => true,
                    _Restart::Never => false,
                    _Restart::UnexpectedExits => false,
                };
                
                if should_restart {
                    println!("[{}] Restart policy active. Relaunching...", prog_name);
                    
                }
                false
                // Le process est mort, on le retire de la liste des vivantss
            }
            Ok(None) => {
                true // Le process tourne encore, on le garde
            }
            Err(e) => {
                println!("Error while checking status of [{}]: {}", prog_name, e); // check qui fail.
                false
            }
		});

        if program.childs.len() < config.num_processes as usize {
            if let _Restart::Always = config.restart_policy {
                 start_prog(program);
            }
        }
	}
}
