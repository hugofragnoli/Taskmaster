use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn read_command(rl: &mut DefaultEditor) -> Option<String> {
    let readline = rl.readline("taskmaster> ");

    match readline {
        Ok(line) => {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                let _ = rl.add_history_entry(trimmed);
                return Some(trimmed.to_string());
            }
            None // Prompt vide + Entrée
        },
        Err(ReadlineError::Interrupted) => {
            println!("SIGINT (Ctrl+C) détecté");
            None
        },
        Err(ReadlineError::Eof) => {
            println!("Déconnexion (Ctrl+D)");
            Some("exit".to_string()) // On simule une commande de sortie
        },
        Err(err) => {
            eprintln!("Erreur de lecture : {:?}", err);
            None
        }
    }
}