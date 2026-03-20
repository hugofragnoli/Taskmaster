use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

// ici result cest pas le std::result::Result classique. Cest un alias defini par rustyline.
// il ressemble a ca : type Result<T> = std::result::Result<T, ReadlineError>;
// Defaulteditor renvoie un ReadlineError en cas dechec.  donc cest ok. (pour nos besoins actuels en tt cas )
pub fn setup_shell(history_path: &str) -> Result<DefaultEditor> {
	let mut rl = DefaultEditor::new()?; // ? pour gerer proprement au lieu de crash si DefaultEditor Fail.
	let _ = rl.load_history(history_path);
	Ok(rl)
}

const PROMPT: &str = "\x1b[34mtask\x1b[0mmas\x1b[31mter >\x1b[0m";

// Option : retourne some("qqchose") si user tjrs la.
// None si user plus la
pub fn read_command(rl: &mut DefaultEditor) -> Option<String> {
    match rl.readline(PROMPT) {
        Ok(line) => {
            let trimmed = line.trim();

            if !trimmed.is_empty() {
                let _ = rl.add_history_entry(trimmed); // stocke dans la RAM. 
            }
            Some(trimmed.to_string())
        }
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
            eprintln!("Ctrl+c or EOF detected. Exiting...");
            
            // LA MODIFICATION EST ICI :
            // Au lieu de renvoyer None, on simule la commande "exit"
            Some("exit".to_string()) 
        }
        Err(err) => {
            eprintln!("Error : {:#?}", err);
            None
        }
    }
}
