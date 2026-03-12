use std::io::Stderr;
use std::os::unix::process::CommandExt;
use std::process::Stdio;
use std::{fs::OpenOptions, process::Command};

use std::time::Instant;

use libc::umask;

use crate::config::structs::{_Restart, Program, Taskmaster};
use crate::{debug, error, info};



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
			info!("[STATUS] {} is alive (PIDs: {:?})", prog_name, pids);
		} else {
			info!("[STATUS] {} is off", prog_name);
		}
	}
}

pub fn start_prog(program: &mut Program) {
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
					umask(mask_val);
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

		match cmd.spawn() {
			Ok(child) => {
				info!("Program [{}] launch with PID {}", prog_name, child.id());
				program.childs.push(child);
			}
			Err(e) => error!("Error during program [{}] launch : {}", prog_name, e),
		}
	}
}

pub fn stop_prog(program: &mut Program) {
	//On recup le signal de la config
	let signal = program.config.1.stop_signal
        .as_ref()
        .map(|s| s.to_libc())
        .unwrap_or(libc::SIGTERM); //sinn SIGTERM par defaut ? 

	//on clear dans check process mtn.
	for child in &mut program.childs {
        let pid = child.id() as i32;
        unsafe {
            libc::kill(pid, signal);
        }
        debug!("Sent signal {} to {}", signal, pid);
    }
}

fn should_relaunch(program: &Program) -> bool {
	let config = &program.config.1;

	if let _Restart::Never = config.restart_policy {
		return false;
	}

	if program.retry_count >= config.max_relauch_retry {
		//  print une seule fois que c'est errorfatal ???
		return false;
	}

	let wait_time = config.minimum_runtime.unwrap_or(1);
	if program.last_launch_time.elapsed().as_secs() < wait_time {
		return false; //attends encore 
	}

	true
}

pub fn check_process_status(taskmaster: &mut Taskmaster) {
	for program in &mut taskmaster.programs {
		let prog_name = &program.config.0.clone();
		let config = &program.config.1;
		program.childs.retain_mut(|child| match child.try_wait() {
			Ok(Some(status)) => {
				let exit_code = status.code();
				info!("[{}] has stopped with status: {}", prog_name, status);

				if program.last_launch_time.elapsed().as_secs() < config.minimum_runtime.unwrap_or(1) {
					program.retry_count += 1;
				} else {
					program.retry_count = 0; // Il a vécu assez longtemps, on reset
				}
				false
				// Le process est mort, on le retire de la liste des vivant(e)s
			}
			Ok(None) => {
				true // Le process tourne encore, on le garde
			}
			Err(e) => {
				error!("Error while checking status of [{}]: {}", prog_name, e); // check qui fail.
				false
			}
		});

		if program.childs.len() < config.num_processes as usize {
			if should_relaunch(program) {
				info!(
					"[{}] Relaunching (Attempt {}/{})",
					prog_name,
					program.retry_count + 1,
					config.max_relauch_retry
				);
				start_prog(program);
			}
		}
	}
}
