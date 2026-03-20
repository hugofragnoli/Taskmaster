use std::os::unix::process::CommandExt;
use std::process::Stdio;
use std::{fs::OpenOptions, process::Command};

use std::time::Instant;

use libc::umask;

use crate::config::structs::{_Restart, Program, Taskmaster};
use crate::{debug, error, info};

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

// j'ai ajouter un bool pour pas que l'exec print quand il fait sa boucle de verification.
// il doit print uniquement quand le main lui envoi une commande.
pub fn start_prog(program: &mut Program, print_message: bool, num_to_start: usize) {
	program.is_stopped_manually = false;
	// ON va mettre un fichier de log par commande ca posera pas de pb dacces DIS MOI CE QUE TEN PENSES BG
	let prog_name = &program.config.0; // "nom du prog bg"
	let args = &program.config.1; // "toute la conf"
	let split_args: Vec<&str> = args.cmd.split_whitespace().collect();

	program.last_launch_time = Instant::now();

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

		// 0666
		// 0022
		// 0644
		// apply umask on child process
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

			//openoptions permet de lui dire dappend plutot que decrire par dessus si on lance 4 proc en mm temps par ex
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
		// debug!("trying to kill process");
		let _result = child.kill();
		// wait necessaire pour tuer le process jsp pourquoi ??
		// kill seul envoi le signal mais si on wait pas ca marche pas
		child.wait().expect("Unable to kill process");
		// println!("kill result: {:?}", result);
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
				// println!({program.unexpected_error_code});
                return false;
            }
        }
        _Restart::Always => (),
    }

	if program.retry_count >= config.max_relauch_retry {
		//  print une seule fois que c'est errorfatal ???
		return false;
	}

	let wait_time = config.minimum_runtime.unwrap_or(1);
	if program.last_launch_time.elapsed().as_secs() < wait_time {
		return false; //attends encore 
	}

	// unexpected error code
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

				// info!("[{}] has stopped with status: {:?}", _prog_name, status);

				if let Some(code) = exit_code {
                    if let Some(errors_code) = &program.config.1.expected_error_codes {
                        if !errors_code.contains(&(code as u32)) {
                            program.unexpected_error_code = true;
                        }
                    }
					else if code != 0 {
						program.unexpected_error_code = true;
					}
				}
				else {
                    program.unexpected_error_code = true; 
                }

				let min_run = program.config.1.minimum_runtime.unwrap_or(1);
				if program.last_launch_time.elapsed().as_secs() < min_run {
                        program.retry_count += 1;
                        error!("[{}] Crashed too quickly (retry {}/{})", 
                            _prog_name, program.retry_count, program.config.1.max_relauch_retry);
				} else {
					program.retry_count = 0; // Il a vécu assez longtemps, on reset
				}
				false
				// Le process est mort, on le retire de la liste des vivant(e)s
			}
			Ok(None) => {
				true // Le process tourne encore, on le garde
			}
			Err(_) => {
				// error!("Error while checking status of [{}]: {}", prog_name, e); // check qui fail.
				false
			}
		});

		let current_len = program.childs.len();
        let target_len = config.num_processes as usize;

        if current_len < target_len && should_relaunch(program) {
            let missing = target_len - current_len; // On calcule ceux qui manquent
            println!("{} {}", _prog_name, current_len);
            info!("[{}] Relaunching {} process (Attempt {})", _prog_name, missing, program.retry_count + 1);
            
            // On ne relance QUE le nombre manquant
            start_prog(program, false, missing);
		}
	}
}
