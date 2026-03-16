use std::{
	ffi::c_void,
	sync::{
		Mutex,
		atomic::{AtomicBool, Ordering},
		mpsc::{RecvTimeoutError, SendError},
	},
	thread::sleep,
	time::Duration,
};

use libc::{SIGHUP, c_int, sighandler_t, signal};

use crate::{
	communication::{self, ThreadMessage},
	config::parser::parse_config,
	error, info,
	taskmasterctl::read_history::read_command,
};

extern crate libc;

// use libc::{ signal};
// use libc::{SIGINT, c_int, c_void};
// use libc::{exit, sighandler_t};

/*
* Global Var
*/

static SIGHUP_RECEIVED: Mutex<AtomicBool> = Mutex::new(AtomicBool::new(false));

/*
* Structs / enums
*/

/// Enum used to handle when the exec thread is stopped unexpectedly
#[derive(Debug)]
enum ThreadShoudQuit {
	Yes,
	No,
}

/*
* Private Functions
*/

/// Used to handle when the exec thread is stopped unexpectedly
/// Check all responses messages from exec_thread
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
		Err(RecvTimeoutError::Disconnected) => match original_message {
			ThreadMessage::Exit => (),
			_ => {
				error!("Unable to receive a response for {:?}: Reason: Disconnected", original_message);
				return ThreadShoudQuit::Yes;
			}
		},
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

extern "C" fn reload_handler(_: c_int) {
	SIGHUP_RECEIVED.lock().unwrap().store(true, Ordering::Relaxed);
}

fn get_handler() -> sighandler_t {
	reload_handler as extern "C" fn(c_int) as *mut c_void as sighandler_t
}

fn setup_reload_handler() {
	unsafe { signal(SIGHUP, get_handler()) };
}

fn should_reload(
	receiver: &std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: &std::sync::mpsc::Sender<communication::ThreadMessage>,
) {
	let guard = SIGHUP_RECEIVED.lock().unwrap();
	if guard.load(Ordering::Relaxed) {
		info!("Reloading Config...");
		let config = parse_config();
		match config {
			Ok(taskmaster) => {
				let _ = sender.send(ThreadMessage::ReloadConfig(taskmaster));
				info!("New config sent to exec thread");

				let received = receiver.recv_timeout(Duration::from_secs(5));
				match received {
					Ok(ThreadMessage::ConfigReloaded) => info!("Config updated."),
					Ok(ThreadMessage::ConfigReloadError(s)) => error!("Unable to reload config: {}", s),
					_ => (),
				}
			}
			Err(e) => {
				error!("Unable to parse new config: {}", e);
			}
		}

		guard.store(false, Ordering::Relaxed); // reset bool
	}
}
/*
* Public Functions
*/

/// Main loop of main thread
/// read input and send instructions to exec thread via ThreadMessage enum
pub fn main_thread_entry(
	receiver: std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: std::sync::mpsc::Sender<communication::ThreadMessage>,
	mut rl: rustyline::Editor<(), rustyline::history::FileHistory>,
) -> Result<(), SendError<ThreadMessage>> {
	setup_reload_handler();

	if !check_exec_ready(&receiver, &sender) {
		return Ok(());
	}
	info!("Execution thread ready.");

	//copie de lancien handle_commands_sh
	loop {
		should_reload(&receiver, &sender);

		let mut should_quit = ThreadShoudQuit::No;

		if let Some(cmd) = read_command(&mut rl) {
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

				["stop", follow_starts @ ..] => {
					for prog_name in follow_starts {
						sender.send(ThreadMessage::Stop(prog_name.to_string()))?;

						info!("Command stop sent.");

						let received = receiver.recv_timeout(Duration::from_secs(5));
						should_quit = handle_response(ThreadMessage::Stop(prog_name.to_string()), received);
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

	Ok(())
}
