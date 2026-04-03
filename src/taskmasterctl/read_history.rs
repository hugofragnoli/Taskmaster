use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub fn setup_shell(history_path: &str) -> Result<DefaultEditor> {
	let mut rl = DefaultEditor::new()?;
	let _ = rl.load_history(history_path);
	Ok(rl)
}

const PROMPT: &str = "\x1b[34mtask\x1b[0mmas\x1b[31mter >\x1b[0m";

pub fn read_command(rl: &mut DefaultEditor) -> Option<String> {
	match rl.readline(PROMPT) {
		Ok(line) => {
			let trimmed = line.trim();

			if !trimmed.is_empty() {
				let _ = rl.add_history_entry(trimmed);
			}
			Some(trimmed.to_string())
		}
		Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
			eprintln!("Ctrl+c or EOF detected. Exiting...");
			Some("exit".to_string())
		}
		Err(err) => {
			eprintln!("Error : {:#?}", err);
			None
		}
	}
}
