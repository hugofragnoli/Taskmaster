//communication pour les signaux bg. si je fais pas ca on peut pas envoyer "quel" signa est envoye et arrte les prog en bg
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StopSig {
    Config,     // Utilise le signal du YAML (ex: SIGTERM)
    Kill,       // SIGKILL direct (brutal)
    Interrupt,  // SIGINT (Ctrl-C)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ThreadMessage {
	// message envoye par le thread main
	Ready,           // envoye par le main thread en attendant que le exec ai demarrer + autostart
	Start(String),   // start un programme identifie par le nom dans la config
	Restart(String), // restart un programme identifie par le nom dans la config
	Stop(String, StopSig),    // stop un programme identifie par le nom dans la config
	Exit,            // ordonne au thread exec de tuer tous les process et de quitter
	StatusAll,       // ordonne au thread exec de print le status de tous les programmes
	Status(String),  // ordonne au thread exec de print le status d'un programme
	// messages envoye par le thread exec
	StatusDone, // reponse du thread exec pour dire qu'il a print le status
	ExitDone,   // reponse du thread exec pour dire qu'il a quitter
	ActionDone, // reponse du thread a start / stop / restart.
	ExecReady,

	StopAll(StopSig),

}
