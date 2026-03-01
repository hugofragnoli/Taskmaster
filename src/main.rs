mod config;
mod logger;

use std::{
	sync::{Arc, Mutex, mpsc::channel},
	thread,
};
mod taskmasterctl;
use config::parser::parse_config;
use taskmasterctl::read_history::read_command;
use taskmasterctl::read_history::setup_shell;

fn main() {
	info!("Starting taskmaster.");
	let taskmaster = parse_config();
	info!("Config parsed");

<<<<<<< HEAD
	// https://doc.rust-lang.org/std/sync/struct.Mutex.html
	let config = Arc::new(Mutex::new(taskmaster));

	let mut threads = Vec::with_capacity(10);
	for i in 0..10 {
		let (data, i) = (Arc::clone(&config), i);
		threads.push(thread::spawn(move || {
			let data = data.lock().unwrap();
			info!(format!(
				"Tread {}: config.program[0].config.0 = {}",
				i, data.programs[0].config.0
			));
		}));
	}

	threads
		.into_iter()
		.for_each(|thread| thread.join().expect("The thread creating or execution failed !"));
=======
	// println!("{:#?}", taskmaster);

	let mut rl = setup_shell("history.txt").expect("Erreur setup");
>>>>>>> 3bb38c6 (correction syntaxe plein de lignes mdr)
}
