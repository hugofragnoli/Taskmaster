use rustyline::{DefaultEditor, Result};
use rustyline::{error::ReadlineError};

//ici result cest pas le std::result::Result classique. Cest un alias defini par rustyline.
// il ressemble a ca : type Result<T> = std::result::Result<T, ReadlineError>;
//Defaulteditor renvoie un ReadlineError en cas dechec.  donc cest ok. (pour nos besoins actuels en tt cas )
pub fn setup_shell(history_path: &str) -> Result <DefaultEditor> {
    let mut rl = DefaultEditor::new()?; // ? pour gerer proprement au lieu de crash si DefaultEditor Fail.
    let _ = rl.load_history(history_path);
    Ok(rl)
}

//Option : retourne some("qqchose") si user tjrs la.
// None si user plus la
pub fn read_command(rl: &mut DefaultEditor) -> Option<String> {
    match rl.readline("taskmaster >") {
        Ok(line) => {
            let trimmed = line.trim();

            if !trimmed.is_empty(){
                let _ = rl.add_history_entry(trimmed); //stocke dans la RAM. 
            }
        //retour  sous forme de stringgg d'anas
        Some(trimmed.to_string())
        },
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
            eprintln!("Ctrl+c Ou EOF detecte");
            None
        },
        Err(err) => {
            eprintln!("Erreur : {:#?}", err);
            None
        }
    }
}