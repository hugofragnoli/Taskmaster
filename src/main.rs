use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

mod exec;
mod read_history;

use exec::exec_and_monitor;
use read_history::read_command;

#[derive(Debug, Serialize, Deserialize)]
enum _Restart {
	Always,
	Never,
	Unexpectedexits,
}

#[derive(Debug, Serialize, Deserialize)]
enum _Signalstopper {
	SIGKILL,
	SIGTERM,
	SIGINT,
}

#[derive(Debug, Serialize, Deserialize)]
enum _Discardoptions {
	Stdin,
	Stdout,
	Stderr,
	FilePath,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProgramConfig {
	cmd: String,
	numprocs: u32,
	autostart: bool,
	status: bool,
	error_code: u32,
	restart: _Restart,
	min_runtime: u64,
	max_relaunch_retry: u32,
	signal_stopper: _Signalstopper,
	time_after_proper_stop: u64,
	discard_options: _Discardoptions,
	env_to_set: HashMap<String, String>,
	working_dir: String,
	umask: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskmasterConfig {
	programs: HashMap<String, ProgramConfig>,
}

fn main() {
	// let f = std::fs::File::open("config.yaml").expect("Fichier introuvable");
	//
	// let config: TaskmasterConfig = serde_yaml::from_reader(f).expect("Erreur de parsing");
	// let serialized = serde_yaml::to_string(&config).unwrap();
	// println!("serialized = {}", serialized);
	// let deserialized: TaskmasterConfig = serde_yaml::from_str(&serialized).unwrap();
	// println!("deserialized = {:#?}", deserialized);
	// if let Some(p) = config.programs.get("my_ping") {
	//     println!("La commande à lancer est : {}", p.cmd);
	// }
	read_command();
	//exec_and_monitor();
}
