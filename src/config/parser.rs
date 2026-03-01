use serde_yaml::from_reader;

use crate::config::structs::ProgramsConfig;

pub fn parse_config() {
	let f = std::fs::File::open("configmael.yaml").expect("Fichier introuvable");

	let config: ProgramsConfig = from_reader(f).expect("Erreur de parsing");
	println!("config {:#?}", config);
}
