use serde_yaml::from_reader;

use crate::config::structs::{Program, ProgramsConfig, Taskmaster};

/// Parse config file
/// 1. Read config file and serialize it
/// 2. Check incompatible value in a program config
/// 3. return TaskMaster struct
pub fn parse_config(path: String) -> Taskmaster {
	let f = std::fs::File::open(path).expect("Fichier introuvable");

	let config: ProgramsConfig = from_reader(f).expect("Erreur de parsing");

	let mut tm: Taskmaster = Taskmaster { programs: vec![] };

	for p in config.programs.iter().enumerate().clone() {
		tm.programs.push(Program {
			config: (p.1.0.clone(), p.1.1.clone()),
			childs: Vec::new(),
		});
	}

	tm
}
