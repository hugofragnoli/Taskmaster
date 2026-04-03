use crate::config::structs::{_Signalstopper, Taskmaster};

#[derive(Debug)]
pub enum ThreadMessage {
	// Messages sent by the main thread
	Ready, // Sent by the main thread while waiting for the exec thread to start + autostart
	Start(String), // Starts a program identified by its name in the config
	Restart(String), // Restarts a program identified by its name in the config
	Stop(String), // Stops a program identified by its name in the config
	Exit,  // Orders the exec thread to kill all processes and exit
	StatusAll, // Orders the exec thread to print the status of all programs
	Status(String), // Orders the exec thread to print the status of a specific program
	ReloadConfig(Taskmaster),

	// Messages sent by the exec thread
	StatusDone, // Response from the exec thread indicating it has printed the status
	ExitDone,   // Response from the exec thread indicating it has exited
	ActionDone, // Response from the exec thread to a start / stop / restart action.
	ExecReady,
	ConfigReloaded,
	SignalReceived(_Signalstopper),
}
