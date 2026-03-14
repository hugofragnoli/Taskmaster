use std::{env::current_exe, time::Duration};

use std::thread::sleep;

use crate::{
	communication::{self, ThreadMessage},
	config::structs::Taskmaster,
	debug, error,
	exec::{check_process_status, print_status, start_prog, stop_prog},
	info,
};

// vec<Signal>

/// Main loop of exec thread
/// Monitor programs
/// Receive instruction from main thread via ThreadMessage enum
pub fn exec_thread_entry(
	receiver: std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: std::sync::mpsc::Sender<communication::ThreadMessage>,
	taskmaster: Taskmaster,
) {
	let mut current_config = taskmaster;

	// start programs at launch
	for program in &mut current_config.programs {
		if program.config.1.autostart {
			for _ in 0..program.config.1.num_processes {
				start_prog(program);
			}
		}
	}
	let _ = sender.send(ThreadMessage::ExecReady);

	loop {
		// handling messages
		while let Ok(msg) = receiver.try_recv() {
			match msg {
				ThreadMessage::Start(cmd) => {
					if let Some(p) = current_config.programs.iter_mut().find(|p| p.config.0 == cmd) {
						if !p.childs.is_empty() {
							info!("Program : '{}' already running.", cmd);
						} else {
							start_prog(p);
						}
					} else {
						error!("Program '{}' not found.", cmd);
					}
					let _ = sender.send(ThreadMessage::ActionDone);
				}
				ThreadMessage::Stop(cmd) => {
					if let Some(p) = current_config.programs.iter_mut().find(|p| p.config.0 == cmd) {
						if !p.childs.is_empty() {
							stop_prog(p);
						}
					} else {
						error!("Program '{}' not found.", cmd);
					}
					let _ = sender.send(ThreadMessage::ActionDone);
				}
				ThreadMessage::Exit => {
					info!("exiting...");
					for program in current_config.programs.iter_mut() {
						for p in program.childs.iter_mut() {
							debug!("Killing {} with pid {}", program.config.0, p.id());
							let _ = p.kill();
							let _ = p.wait();
						}
					}
					let _ = sender.send(ThreadMessage::ExitDone);
					return; //return plutot que break pour bien quittter la fonction et detruire le thread exec.
				}
				ThreadMessage::StatusAll => {
					print_status(&current_config, None);
					let _ = sender.send(ThreadMessage::StatusDone);
				}
				ThreadMessage::Status(cmd) => {
					print_status(&current_config, Some(&cmd));
					let _ = sender.send(ThreadMessage::StatusDone);
				}
				ThreadMessage::Restart(cmd) => {
					if let Some(p) = current_config.programs.iter_mut().find(|p| p.config.0 == cmd) {
						if p.childs.is_empty() {
							info!("Program : '{}' already off.", cmd);
						} else {
							stop_prog(p);
							start_prog(p);
						}
					} else {
						error!("Program '{}' not found.", cmd);
					}
					let _ = sender.send(ThreadMessage::ActionDone);
				}
				ThreadMessage::ReloadConfig(taskmaster) => {
					// kill -s SIGHUP $(ps aux | grep Taskmaster | awk '{ print $2 }')
					for program in taskmaster.programs {
						if let Some(candidat) = current_config.programs.iter().find(|&p| p.config.0 == program.config.0)
						{
							println!("{} is in the 2 configs", candidat.config.0);
							if candidat.config.1 != program.config.1 {
								println!("Config changed for : {}", candidat.config.0);
							}
						}
					}

					let _ = sender.send(ThreadMessage::ConfigReloaded);
				}
				_ => (),
			}
		}
		check_process_status(&mut current_config);
		sleep(Duration::from_millis(100));
	}
}
