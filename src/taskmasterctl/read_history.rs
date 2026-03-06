use rustyline::{DefaultEditor, Result};
use rustyline::{error::ReadlineError};

// ici result cest pas le std::result::Result classique. Cest un alias defini par rustyline.
// il ressemble a ca : type Result<T> = std::result::Result<T, ReadlineError>;
// Defaulteditor renvoie un ReadlineError en cas dechec.  donc cest ok. (pour nos besoins actuels en tt cas )
pub fn setup_shell(history_path: &str) -> Result <DefaultEditor> {
    let mut rl = DefaultEditor::new()?; // ? pour gerer proprement au lieu de crash si DefaultEditor Fail.
    let _ = rl.load_history(history_path);
    Ok(rl)
}

const BLUE: &str = "\x1b[34m";
const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";

const PROMPT: &str = "\x1b[34mtask\x1b[0mmas\x1b[31mter >\x1b[0m";

// Option : retourne some("qqchose") si user tjrs la.
// None si user plus la
pub fn read_command(rl: &mut DefaultEditor) -> Option<String> {
    // let prompt = "\x01\x1b[94m\x02task\x01\x1b[97m\x02mas\x01\x1b[91m\x02ter > \x01\x1b[0m\x02";
    match rl.readline(PROMPT) {
        Ok(line) => {
            let trimmed = line.trim();
            
            if !trimmed.is_empty(){
                let _ = rl.add_history_entry(trimmed); //stocke dans la RAM. 
            }
            // println!("{}", trimmed.to_string()); DEBUG BG
        //retour  sous forme de stringgg d'anas
        Some(trimmed.to_string())
        },
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
            eprintln!("Ctrl+c or EOF detected");
            None
        },
        Err(err) => {
            eprintln!("Error : {:#?}", err);
            None
        }
    }
}
