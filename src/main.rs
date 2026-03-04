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
use exec::{start_prog, check_process_status};
use taskmasterctl::read_history::read_command;
use taskmasterctl::read_history::setup_shell;

fn exec_thread_entry(
	receiver: std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: std::sync::mpsc::Sender<communication::ThreadMessage>,
	mut taskmaster: Taskmaster,
) {
	loop {
		// handling messages
		while let Ok(msg) = receiver.try_recv() {
			match msg {
				ThreadMessage::Start(cmd) => {
					if let Some(p) = taskmaster.programs.iter_mut().find(|p| p.config.0 == cmd) {
						if !p.childs.is_empty() {
							println!("Program : '{}' already running.", cmd);
						} else {
							start_prog(p); 
						}
					} else {
						println!("Error: Program '{}' not found.", cmd);
					}
				}
				ThreadMessage::Exit => {
					println!("exiting...");
					return; //break plutot que return pour bien quittter la fonction et detruire le thread exec.
				}
				_ => println!("CACA"),
			}
		}
		check_process_status(&mut taskmaster);
		sleep(Duration::from_millis(100));
	}
}

fn main_thread_entry(
	receiver: std::sync::mpsc::Receiver<communication::ThreadMessage>,
	sender: std::sync::mpsc::Sender<communication::ThreadMessage>,
	mut rl: rustyline::Editor<(), rustyline::history::FileHistory>,
) {
	loop {
		if let Some(cmd) = read_command(&mut rl) {
			let splitted: Vec<&str> = cmd.split_whitespace().collect();
            
            match &splitted[..] {
                ["start" | "restart", follow_starts @ ..] => {
                    for prog_name in follow_starts {
                        let res = sender.send(ThreadMessage::Start(prog_name.to_string()));
					println!("Command start sent: {:?}", res);
					}
				}
				["exit"] => {
                    let res = sender.send(ThreadMessage::Exit);
                    println!("Commande exit sent...");
                    sleep(Duration::from_secs(4)); // Sleep en attendant quon ferme tout ? 
                    break;
				}
				["status"] => {
                    // C'est ici qu'on enverra ThreadMessage::StatusAll 
                    println!("status request sent...");
                }
                _ => {
                    if !cmd.trim().is_empty() {
                        println!("Error : Invalid command or missing argument(s) : {}", cmd);
                    }
                }
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

	let thread_exec = thread::spawn(move || {
		exec_thread_entry(main_to_exec_receiver, exec_to_main_sender, taskmaster)
	});
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
