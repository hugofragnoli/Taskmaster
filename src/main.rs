use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod config;

use config::parser::parse_config;
use config::structs::ProgramConfig;

mod exec;

use exec::exec_and_monitor;

fn main() {
	// let f = std::fs::File::open("config.yaml").expect("Fichier introuvable");
	//
	// let config: TaskmasterConfig = serde_yaml::from_reader(f).expect("Erreur de parsing");
	// let serialized = serde_yaml::to_string(&config).unwrap();
	// println!("serialized = {}", serialized);
	// let deserialized: TaskmasterConfig = serde_yaml::from_str(&serialized).unwrap();
	// println!("deserialized = {:#?}", deserialized);
	// if let Some(p) = config.programs.get("my_ping") {
	// 	println!("La commande à lancer est : {}", p.cmd);
	// }

	// exec_and_monitor();

	parse_config();
}
