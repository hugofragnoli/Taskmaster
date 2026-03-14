use std::{
	sync::mpsc::{RecvTimeoutError, SendError},
	thread::{Thread, sleep},
	time::Duration,
};

use crate::{
	communication::{self, ThreadMessage, StopSig},
	error, info,
	taskmasterctl::read_history::read_command,
};

#[derive(Debug)]
enum ThreadShoudQuit {
	Yes,
	No,
}

fn handle_response(
	original_message: ThreadMessage,
	response: Result<ThreadMessage, RecvTimeoutError>,
) -> ThreadShoudQuit {
	match response {
		Ok(ThreadMessage::ActionDone) => info!("Done."),
		Ok(ThreadMessage::ExitDone) => info!("Exec thread successfully quit."),
		Ok(ThreadMessage::StatusDone) => (),
		Err(RecvTimeoutError::Timeout) => {
			error!("Unable to receive a response for {:?}. Reason: Timeout", original_message)
		}
		Err(RecvTimeoutError::Disconnected) => {
			if ThreadMessage::Exit != original_message {
				error!("Unable to receive a response for {:?}: Reason: Disconnected", original_message);
				return ThreadShoudQuit::Yes;
			}
		}
		_ => (),
	}
	ThreadShoudQuit::No
}

fn check_exec_ready(
	receiver: &std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: &std::sync::mpsc::Sender<communication::ThreadMessage>,
) -> bool {
	sleep(Duration::from_secs(2));
	let res = sender.send(ThreadMessage::Ready);
	if res.is_err() {
		return false;
	}

	let received = receiver.recv_timeout(Duration::from_secs(5));
	match received {
		Ok(ThreadMessage::ExecReady) => true,
		_ => {
			error!("Exec thread don't respond.");
			false
		}
	}
}

/// Main loop of main thread
/// read input and send instructions to exec thread via ThreadMessage enum
pub fn main_thread_entry(
	receiver: std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: std::sync::mpsc::Sender<communication::ThreadMessage>,
	mut rl: rustyline::Editor<(), rustyline::history::FileHistory>,
) -> Result<(), SendError<ThreadMessage>> {
	if !check_exec_ready(&receiver, &sender) {
		return Ok(());
	}
	info!("Execution thread ready.");

	//copie de lancien handle_commands_sh
	loop {
		let mut should_quit = ThreadShoudQuit::No;

		while let Some(cmd) = read_command(&mut rl) {
			let splitted: Vec<&str> = cmd.split_whitespace().collect();
			//ajout du sighandler TODO
			match &splitted[..] {
				["start", follow_starts @ ..] => {
					for prog_name in follow_starts {
						sender.send(ThreadMessage::Start(prog_name.to_string()))?;

						info!("Command start sent.");

						let received = receiver.recv_timeout(Duration::from_secs(5));
						should_quit = handle_response(ThreadMessage::Start(prog_name.to_string()), received);
					}
				}

				["stop", follow_stops @ ..] => {
					for prog_name in follow_stops {
						sender.send(ThreadMessage::Stop(prog_name.to_string(), StopSig::Config))?;

						info!("Command stop sent.");

						let received = receiver.recv_timeout(Duration::from_secs(5));
								// a finir
						should_quit = handle_response(
							ThreadMessage::Stop(prog_name.to_string(), StopSig::Config), 
							received
						);
					}
				}

				["exit"] => {
					sender.send(ThreadMessage::Exit)?;
					info!("Command exit sent.");

					sleep(Duration::from_secs(1)); // Sleep en attendant quon ferme tout ? oui sidi

					let received = receiver.recv_timeout(Duration::from_secs(5));
					handle_response(ThreadMessage::Exit, received);
					break;
				}
				["restart", follow_starts @ ..] => {
					for prog_name in follow_starts {
						sender.send(ThreadMessage::Restart(prog_name.to_string()))?;

						info!("Command restart sent.");

						let received = receiver.recv_timeout(Duration::from_secs(5));
						should_quit = handle_response(ThreadMessage::Restart(prog_name.to_string()), received);
					}
				}
				["status"] => {
					sender.send(ThreadMessage::StatusAll)?;

					info!("status request sent...");

					let received = receiver.recv_timeout(Duration::from_secs(5));
					should_quit = handle_response(ThreadMessage::StatusAll, received);
				}
				["status", follow_status @ ..] => {
					for prog_name in follow_status {
						sender.send(ThreadMessage::Status(prog_name.to_string()))?;
						let received = receiver.recv_timeout(Duration::from_secs(5));
						should_quit = handle_response(ThreadMessage::Status(prog_name.to_string()), received);
					}
				}
				["I just received a Ctrl-C. Let me check if i have to stop some programs..."] => {
					sender.send(ThreadMessage::StopAll(communication::StopSig::Kill))?;
    
					let received = receiver.recv_timeout(Duration::from_secs(5));
					handle_response(ThreadMessage::StopAll(communication::StopSig::Kill), received);
				}
				["clear"] => {
					let _ = rl.clear_screen();
				}
				_ => {
					if !cmd.trim().is_empty() {
						println!("Error : Invalid command or missing argument(s) : {}", cmd);
					}
				}
			}



			if let ThreadShoudQuit::Yes = should_quit {
				break;
			}
		}
	}
	info!("Closing Taskmaster, Bye bye :)..");
    
    // On envoie l'ordre de tout tuer
    sender.send(ThreadMessage::Exit)?;

    // On attend que l'Exec confirme qu'il a bien kill/wait tout le monde
    let received = receiver.recv_timeout(Duration::from_secs(6));
    handle_response(ThreadMessage::Exit, received);

	Ok(())
}
