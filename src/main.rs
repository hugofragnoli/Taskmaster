use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::time::Duration;

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
    
}

#[derive(Debug, Serialize, Deserialize)]
struct TaskmasterConfig {
    programs: HashMap<String, ProgramConfig>,
}

fn main() {
    let f = std::fs::File::open("config.yaml").expect("Fichier introuvable");

    let config: TaskmasterConfig = serde_yaml::from_reader(f).expect("Erreur de parsing");
    let serialized = serde_yaml::to_string(&config).unwrap();
    println!("serialized = {}", serialized);
    let deserialized: TaskmasterConfig = serde_yaml::from_str(&serialized).unwrap();
    println!("deserialized = {:#?}", deserialized);
    if let Some(p) = config.programs.get("my_ping") {
        println!("La commande à lancer est : {}", p.cmd);
    }
}
