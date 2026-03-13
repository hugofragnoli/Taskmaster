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
	mut taskmaster: Taskmaster,
) {
	// start programs at launch
	for program in &mut taskmaster.programs {
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
					if let Some(p) = taskmaster.programs.iter_mut().find(|p| p.config.0 == cmd) {
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
					if let Some(p) = taskmaster.programs.iter_mut().find(|p| p.config.0 == cmd) {
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
					for program in taskmaster.programs.iter_mut() {
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
					print_status(&taskmaster, None);
					let _ = sender.send(ThreadMessage::StatusDone);
				}
				ThreadMessage::Status(cmd) => {
					print_status(&taskmaster, Some(&cmd));
					let _ = sender.send(ThreadMessage::StatusDone);
				}
				ThreadMessage::Restart(cmd) => {
					if let Some(p) = taskmaster.programs.iter_mut().find(|p| p.config.0 == cmd) {
						if p.childs.is_empty() {
							info!("Program : '{}' already off, we gonna start only.", cmd);
							start_prog(p);
						} else {
							stop_prog(p);

							for child in &mut p.childs {
								let _ = child.wait(); 
							}
							p.childs.clear();
							start_prog(p);
						}
					} else {
						error!("Program '{}' not found.", cmd);
					}
					let _ = sender.send(ThreadMessage::ActionDone);
				}
				_ => (),
			}
		}
		check_process_status(&mut taskmaster);
		sleep(Duration::from_millis(100));
	}
}
