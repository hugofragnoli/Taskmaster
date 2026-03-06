pub enum ThreadMessage {
	// message envoye par le thread main
	Start(String),   // start un programme identifie par le nom dans la config
	Restart(String), // restart un programme identifie par le nom dans la config
	Stop(String),    // stop un programme identifie par le nom dans la config
	Exit,            // ordonne au thread exec de tuer tous les process et de quitter
	StatusAll,       // ordonne au thread exec de print le status de tous les programmes
	Status(String),          // ordonne au thread exec de print le status d'un programme
	// messages envoye par le thread exec
	StatusDone, // reponse du thread exec pour dire qu'il a print le status
	ExitDone,   // reponse du thread exec pour dire qu'il a quitter
	ActionDone, // reponse du thread a start / stop / restart.
}

// exec
// loop {
// if !tunnel.empty() {
// match tunnel.pop() {
//
// }
// }

// }
