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
use crate::config::structs::Taskmaster;
use crate::{communication::ThreadMessage, config::parser::parse_config};
use exec::{start_prog, check_process_status};
use taskmasterctl::read_history::read_command;
use taskmasterctl::read_history::setup_shell;

//ajout de taskmaster en param pour recup du main.
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
	//copie de lancien handle_commands_sh
	loop {
		if let Some(cmd) = read_command(&mut rl) {
			let splitted: Vec<&str> = cmd.split_whitespace().collect();
            //ajout du sighandler TODO
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
                    sleep(Duration::from_secs(1)); // Sleep en attendant quon ferme tout ? 
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
	//on utilise move pour transferer le ownership au thread exec. Le thread exec recupere tout droit sur la struct taskmaster
	//thread main na plus le droit de lutiliser, de le lire ou de le modif.
	let thread_exec = thread::spawn(move || {
		exec_thread_entry(main_to_exec_receiver, exec_to_main_sender, taskmaster)
	});
	main_thread_entry(exec_to_main_receiver, main_to_exec_sender, rl);
}
