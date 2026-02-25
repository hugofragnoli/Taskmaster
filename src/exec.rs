use std::{
	fs::File,
	process::{Command, Stdio},
	thread::sleep,
	time::Duration,
};

// redirige stdio du premier process dans stdin du second
fn pipe_process() {
	let ls = Command::new("ls")
		.stdout(Stdio::piped())
		.spawn()
		.expect("Failed to execute ls");

	let ls_stdout = ls.stdout.expect("failed to pipe stdout of ls process");

	let cat = Command::new("cat")
		.stdin(Stdio::from(ls_stdout))
		.stdout(Stdio::piped())
		.spawn()
		.expect("failed to execute cat");

	let cat_stdout = cat
		.wait_with_output()
		.expect("failed to pipe stdout of cat process");

	println!("{}", String::from_utf8_lossy(cat_stdout.stdout.as_slice()));
}

// simule un programme long avec redirection d'output dans un fichier de log.
fn redirect_to_file() {
	let logfile = File::create("logfile.txt").expect("failed to open file");
	let mut ping = Command::new("ping")
		.stdout(logfile)
		.args(&["google.com"])
		.spawn()
		.expect("failed to start ping");

	sleep(Duration::from_mins(2)); // sleep 2 minutes
	ping.kill().expect("Failed to kill ping");
}

pub fn exec_and_monitor() {
	pipe_process();

	redirect_to_file();
}
