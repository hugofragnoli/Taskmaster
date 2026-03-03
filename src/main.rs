mod config;
mod logger;

use std::{
	sync::{Arc, Mutex, mpsc::channel},
	thread::{self, sleep},
	time::Duration,
};
mod communication;
mod exec;
mod taskmasterctl;
//use config::parser::parse_config;
use crate::config::structs::{Program, ProgramConfig2, Taskmaster};
use crate::{communication::ThreadMessage, config::parser::parse_config};
use exec::handle_commands_sh;
use taskmasterctl::read_history::read_command;
use taskmasterctl::read_history::setup_shell;

fn exec_thread_entry(
	receiver: std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: std::sync::mpsc::Sender<communication::ThreadMessage>,
) {
	loop {
		// handling messages
		if let Ok(msg) = receiver.try_recv() {
			match msg {
				ThreadMessage::Start(cmd) => println!("Exec thread received starting cmd for {}", cmd),
				ThreadMessage::Exit => {
					println!("exiting...");
					break;
				}
				_ => println!("merde"),
			}
		}

		// check status of program...
		sleep(Duration::from_secs(2));
	}
}

fn main_thread_entry(
	receiver: std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: std::sync::mpsc::Sender<communication::ThreadMessage>,
	mut rl: rustyline::Editor<(), rustyline::history::FileHistory>,
) {
	loop {
		if let Some(cmd) = read_command(&mut rl) {
			if cmd.trim_start().starts_with("start") {
				let res = sender.send(ThreadMessage::Start("ping".to_string()));
				println!("Command start sent: {:?}", res);
			} else if cmd.trim_start().starts_with("exit") {
				let res = sender.send(ThreadMessage::Exit);
				println!("Command exit sent: {:?}", res);
				sleep(Duration::from_secs(4));
				break;
			}
		}
	}
}

//rl_save_history a la fin de la boucle :
// si fichier nexiste pas : le cree et y ecrit lhistorique de la session
// sil existe: ecrase ou le met a jour avec les nouvelles commandes.
fn main() {
	let mut taskmaster = parse_config();

	// println!("{:#?}", taskmaster);

	let path = "history.txt";

	let mut rl = match setup_shell(path) {
		Ok(editor) => editor,
		Err(_) => return,
	};

	// main_to_exec
	let (main_to_exec_sender, main_to_exec_receiver) = channel::<ThreadMessage>();

	// exec_to_main
	let (exec_to_main_sender, exec_to_main_receiver) = channel::<ThreadMessage>();

	let thread_exec = thread::spawn(|| exec_thread_entry(main_to_exec_receiver, exec_to_main_sender));

	main_thread_entry(exec_to_main_receiver, main_to_exec_sender, rl);

	// while let Some(line) = read_command(&mut rl) {
	// 	if line.trim_start().starts_with("exit") {
	// 		//Sortir propre ici // TODO
	// 		break;
	// 	}
	// 	if line.is_empty() {
	// 		continue;
	// 	}
	// 	// ici faut quon envoie la config + la line a handle commands comme ca il gere tout direct
	// 	if line.trim_start().starts_with("status")
	// 		|| line.trim_start().starts_with("start")
	// 		|| line.trim_start().starts_with("stop")
	// 		|| line.trim_start().starts_with("restart")
	// 	{
	// 		handle_commands_sh(&line, &mut taskmaster);
	// 		continue;
	// 	} else {
	// 		println!("Unknown or unrecognized command bro : {}", line);
	// 		continue;
	// 	}
	// }
	// let _ = rl.save_history(path);
}
