use std::time::Duration;

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
			start_prog(program, true, program.config.1.num_processes as usize);
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
							start_prog(p, true, p.config.1.num_processes as usize);
						}
					} else {
						error!("Program '{}' not found.", cmd);
					}
					let _ = sender.send(ThreadMessage::ActionDone);
				}
				ThreadMessage::SignalReceived(received_sig) => {
					info!("Exec : Réception du signal {:?}. Analyse de la configuration...", received_sig);
					
					for program in current_config.programs.iter_mut() {
						if let Some(config_sig) = &program.config.1.stop_signal {
							if *config_sig == received_sig {
								info!("Signal {:?} corresponding to '{}''s config. Stoping it properly...", received_sig, program.config.0);
								
								if !program.childs.is_empty() {
									stop_prog(program);
								}
							}
						}
					}
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
							start_prog(p, true, p.config.1.num_processes as usize);
						}
					} else {
						error!("Program '{}' not found.", cmd);
					}
					let _ = sender.send(ThreadMessage::ActionDone);
				}
				ThreadMessage::ReloadConfig(new_config) => {
					// kill -s SIGHUP $(ps aux | grep Taskmaster | awk '{ print $2 }')

					let mut to_remove: Vec<usize> = vec![];

					for (index, program) in current_config.programs.iter_mut().enumerate() {
						// removed program
						if !new_config.programs.contains(program) {
							// program is not present in new config
							to_remove.push(index);
							info!("{} will be removed.", program.config.0);
						} else {
							let new_p = new_config.programs.iter().find(|p| p.config.0 == program.config.0).unwrap();

							println!("{} is in the 2 configs", new_p.config.0);
							if new_p.config.1 != program.config.1 {
								println!("Config changed for : {}", program.config.0);

								// kill process if running
								if !program.childs.is_empty() {
									stop_prog(program);
								}
								program.config.1 = new_p.config.1.clone();
								info!("{} updated.", new_p.config.0);
								// restart if necessary
								if program.config.1.autostart {
									start_prog(program, true, program.config.1.num_processes as usize);
								}
							}
						}
					}

					// added program
					for program in new_config.programs {
						if !current_config.programs.contains(&program) {
							info!("New program in config {}", program.config.0);
							current_config.programs.push(program);
							if let Some(p) = current_config.programs.last_mut()
								&& p.config.1.autostart
							{
								start_prog(p, true, p.config.1.num_processes as usize);
							}
						}
					}

					// kill and remove
					for idx in to_remove {
						if !current_config.programs[idx].childs.is_empty() {
							stop_prog(&mut current_config.programs[idx]);
						}
						current_config.programs.remove(idx);
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
