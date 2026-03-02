use std::{collections::HashMap, process::Child};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum _Restart {
	Always,
	Never,
	UnexpectedExits,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum _Signalstopper {
	Sigkill,
	Sigterm,
	Sigint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum _Discardoptions {
	Stdin,
	Stdout,
	Stderr,
	FilePath,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum _Shelloptions {
	Start,
	Stop,
	Status,
	Exit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Redirect {
	// filepaths
	pub stdout: String,
	pub stderr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramConfig2 {
	pub cmd: String,                                 // command to run
	pub num_processes: u32,                          // process to start and keep running
	pub autostart: bool,                             // launch program at start of taskmaster
	pub restart_policy: _Restart,                    // always|never|unexpected exit
	pub expected_error_codes: Option<Vec<u32>>,      // normal exit codes
	pub minimum_runtime: Option<u64>,                // minimum time to consider the program "successfully started"
	pub max_relauch_retry: u32,                      // how many restart before abortting
	pub stop_signal: Option<_Signalstopper>,         // signal used to stop the program
	pub time_after_proper_stop: Option<u64>,         // time to wait before killing the program if stop signal didn't work
	pub redirect: Option<Redirect>,                  // redirect stdout/stderr to file or to trash if None
	pub env_to_set: Option<HashMap<String, String>>, // env var to set
	pub working_dir: Option<String>,                 // working directory to set
	pub umask: Option<u32>,                          // umask to set before starting
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramsConfig {
	pub programs: HashMap<String, ProgramConfig2>,
}

#[derive(Debug)]
pub struct Program {
	pub config: (String, ProgramConfig2),
	pub childs: Vec<Child>,
}

#[derive(Debug)]
pub struct Taskmaster {
	pub programs: Vec<Program>,
}
