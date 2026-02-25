use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct ProgramConfig {
    cmd: String,
    numprocs: u32,
    autostart: bool,
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
    println!("deserialized = {:?}", deserialized);
    if let Some(p) = config.programs.get("my_ping") {
        println!("La commande à lancer est : {}", p.cmd);
    }
}
 