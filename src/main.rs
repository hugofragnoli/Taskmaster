mod config;
mod logger;

use std::{sync::mpsc::channel, thread};
mod communication;
mod exec;
mod taskmasterctl;
mod threads;

use threads::exec_entry::exec_thread_entry;
use threads::main_entry::main_thread_entry;

//use config::parser::parse_config;
use crate::{communication::ThreadMessage, config::parser::parse_config};
use taskmasterctl::read_history::setup_shell;

//ajout de taskmaster en param pour recup du main.

fn main() {
	let taskmaster = parse_config();

	// println!("{:#?}", taskmaster);

	let path = "history.txt";

	let rl = match setup_shell(path) {
		Ok(editor) => editor,
		Err(_) => return,
	};

	// main_to_exec
	let (main_to_exec_sender, main_to_exec_receiver) = channel::<ThreadMessage>();

	// exec_to_main
	let (exec_to_main_sender, exec_to_main_receiver) = channel::<ThreadMessage>();
	//on utilise move pour transferer le ownership au thread exec. Le thread exec recupere tout droit sur la struct taskmaster
	//thread main na plus le droit de lutiliser, de le lire ou de le modif.

	let thread_exec = thread::spawn(move || exec_thread_entry(main_to_exec_receiver, exec_to_main_sender, taskmaster));

	let _ = main_thread_entry(exec_to_main_receiver, main_to_exec_sender, rl);
}
