mod config;
mod logger;

use std::{sync::mpsc::channel, thread};
mod communication;
mod errors;
mod exec;
mod taskmasterctl;
mod threads;

use threads::exec_entry::exec_thread_entry;
use threads::main_entry::main_thread_entry;

use crate::{communication::ThreadMessage, config::parser::parse_config};
use taskmasterctl::read_history::setup_shell;

fn main() {
	let config = parse_config();

	match config {
		Ok(taskmaster) => {
			let path = "history.txt";

			let rl = match setup_shell(path) {
				Ok(editor) => editor,
				Err(_) => return,
			};

			// main_to_exec
			let (main_to_exec_sender, main_to_exec_receiver) = channel::<ThreadMessage>();

			// exec_to_main
			let (exec_to_main_sender, exec_to_main_receiver) = channel::<ThreadMessage>();

			let thread_exec = thread::spawn(move || {
				exec_thread_entry(main_to_exec_receiver, exec_to_main_sender, taskmaster)
			});

			let _ = main_thread_entry(exec_to_main_receiver, main_to_exec_sender, rl);

			let _ = thread_exec.join();
		}
		Err(e) => {
			critical!("{}", e);
		}
	}
}
