mod config;
mod logger;

use std::{
	sync::{Arc, Mutex, mpsc::channel},
	thread,
};
mod taskmasterctl;
use config::parser::parse_config;
use libc::read;
use taskmasterctl::read_history::read_command;
use taskmasterctl::read_history::setup_shell;


//rl_save_history a la fin de la boucle : 
// si fichier nexiste pas : le cree et y ecrit lhistorique de la session
// sil existe: ecrase ou le met a jour avec les nouvelles commandes.
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

	let path = "history.txt";

	let mut rl = match setup_shell(path) {
		Ok(editor) => editor,
        Err(_) => return,
	};

	while let Some(line) = read_command(&mut rl) {
		if line == "exit" {
			break;
		}
		if line.is_empty() {
			continue;
		}
		if line.starts_with("status") || line.starts_with("start") || line.starts_with("stop") {
			// handle_commands(line);
			continue;
		}
		else {
			println!("Commande inconnue bro : {}", line);
			continue;
		}
	}

	let _ = rl.save_history(path);
}
