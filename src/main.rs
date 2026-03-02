mod config;
mod logger;

use std::{
	sync::{Arc, Mutex, mpsc::channel},
	thread,
};
mod taskmasterctl;
mod exec;
//use config::parser::parse_config;
use taskmasterctl::read_history::read_command;
use taskmasterctl::read_history::setup_shell;
use exec::handle_commands_sh;


//rl_save_history a la fin de la boucle : 
// si fichier nexiste pas : le cree et y ecrit lhistorique de la session
// sil existe: ecrase ou le met a jour avec les nouvelles commandes.
fn main() {
	// let taskmaster = parse_config();

	// println!("{:#?}", taskmaster);

	let path = "history.txt";

	let mut rl = match setup_shell(path) {
		Ok(editor) => editor,
        Err(_) => return,
	};

	while let Some(line) = read_command(&mut rl) {
		if line.trim_start().starts_with("exit") {
			//Sortir propre ici // TODO
			break;
		}
		if line.is_empty() {
			continue;
		}
		// ici faut quon envoie la config + la line a handle commands comme ca il gere tout direct
		if line.trim_start().starts_with("status") || line.trim_start().starts_with("start") || line.trim_start().starts_with("stop") || line.trim_start().starts_with("restart"){
			 // handle_commands_sh(&taskmaster, line); (A CREER) DOIT PRENDRE TASKMASTER EN PARAM. +  LINE pour comparer
			continue;
		}
		else {
			println!("Commande inconnue bro : {}", line);
			continue;
		}
	}
	let _ = rl.save_history(path); 
}
