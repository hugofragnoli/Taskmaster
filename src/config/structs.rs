use std::{collections::HashMap, process::Child};

use serde::{Deserialize, Deserializer, Serialize, de::Error};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum _Restart {
	Always,
	Never,
	UnexpectedExits,
}

// https://faculty.cs.niu.edu/~hutchins/csci480/signals.htm
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum _Signalstopper {
	SIGHUP, // reload config
	SIGINT,
	SIGQUIT,
	SIGILL,
	SIGTRAP,
	SIGABRT,
	SIGIOT,
	SIGBUS,
	SIGFPE,
	SIGKILL,
	SIGUSR1,
	SIGSEGV,
	SIGUSR2,
	SIGPIPE,
	SIGALRM,
	SIGTERM,
	SIGSTKFLT,
	SIGCHLD,
	SIGCONT,
	SIGSTOP,
	SIGTSTP,
	SIGTTIN,
	SIGTTOU,
	SIGURG,
	SIGXCPU,
	SIGXFSZ,
	SIGVTALRM,
	SIGPROF,
	SIGWINCH,
	SIGIO,
	SIGPOLL,
	SIGPWR,
	SIGSYS,
	SIGUNUSED,
}

impl _Signalstopper {
	pub fn from_i32(n: i32) -> Option<Self> {
		match n {
			libc::SIGHUP => Some(_Signalstopper::SIGHUP),
			libc::SIGINT => Some(_Signalstopper::SIGINT),
			libc::SIGQUIT => Some(_Signalstopper::SIGQUIT),
			libc::SIGILL => Some(_Signalstopper::SIGILL),
			libc::SIGTRAP => Some(_Signalstopper::SIGTRAP),
			libc::SIGABRT => Some(_Signalstopper::SIGABRT),
			libc::SIGIOT => Some(_Signalstopper::SIGIOT),
			libc::SIGBUS => Some(_Signalstopper::SIGBUS),
			libc::SIGFPE => Some(_Signalstopper::SIGFPE),
			libc::SIGKILL => Some(_Signalstopper::SIGKILL),
			libc::SIGUSR1 => Some(_Signalstopper::SIGUSR1),
			libc::SIGSEGV => Some(_Signalstopper::SIGSEGV),
			libc::SIGUSR2 => Some(_Signalstopper::SIGUSR2),
			libc::SIGPIPE => Some(_Signalstopper::SIGPIPE),
			libc::SIGALRM => Some(_Signalstopper::SIGALRM),
			libc::SIGTERM => Some(_Signalstopper::SIGTERM),
			libc::SIGCHLD => Some(_Signalstopper::SIGCHLD),
			libc::SIGCONT => Some(_Signalstopper::SIGCONT),
			libc::SIGSTOP => Some(_Signalstopper::SIGSTOP),
			libc::SIGTSTP => Some(_Signalstopper::SIGTSTP),
			libc::SIGTTIN => Some(_Signalstopper::SIGTTIN),
			libc::SIGTTOU => Some(_Signalstopper::SIGTTOU),
			libc::SIGURG => Some(_Signalstopper::SIGURG),
			libc::SIGXCPU => Some(_Signalstopper::SIGXCPU),
			libc::SIGXFSZ => Some(_Signalstopper::SIGXFSZ),
			libc::SIGVTALRM => Some(_Signalstopper::SIGVTALRM),
			libc::SIGPROF => Some(_Signalstopper::SIGPROF),
			libc::SIGWINCH => Some(_Signalstopper::SIGWINCH),
			libc::SIGIO => Some(_Signalstopper::SIGIO),
			libc::SIGSYS => Some(_Signalstopper::SIGSYS),
			_ => None,
		}
	}
}

impl From<u32> for _Signalstopper {
	fn from(value: u32) -> Self {
		todo!()
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct Redirect {
	// filepaths
	pub stdout: String,
	pub stderr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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
	#[serde(default, deserialize_with = "deserialize_umask")]
	pub umask: Option<u16>, // umask to set before starting
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramsConfig {
	pub programs: HashMap<String, ProgramConfig2>,
}

#[derive(Debug)]
pub struct Program {
	// (name of program, config of program)
	pub config: (String, ProgramConfig2),
	pub childs: Vec<Child>,
	pub retry_count: u32,
	pub last_launch_time: std::time::Instant,
	pub unexpected_error_code: bool,
}

impl PartialEq for Program {
	fn eq(&self, other: &Self) -> bool {
		self.config.0 == other.config.0
	}
}

#[derive(Debug)]
pub struct Taskmaster {
	pub programs: Vec<Program>,
	pub config_file: String,
}

/// custom deserializer for umask option
/// deserialize to u16 if the user put 0o022 or "022" in the config file
fn deserialize_umask<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
where
	D: Deserializer<'de>,
{
	#[derive(Deserialize)]
	#[serde(untagged)]
	enum UmaskConfig {
		Str(String),
		Int(u16),
	}

	match Option::<UmaskConfig>::deserialize(deserializer)? {
		Some(UmaskConfig::Str(s)) => {
			let clean_s = s.trim_start_matches("0o");
			u16::from_str_radix(clean_s, 8).map(Some).map_err(Error::custom)
		}
		Some(UmaskConfig::Int(i)) => Ok(Some(i)),
		None => Ok(None),
	}
}
