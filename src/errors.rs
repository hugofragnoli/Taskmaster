use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum TaskmasterError {
	ParsingError(String), // if error in config file
	InvalidParam(String), // invalid param in config
	Argument(String),     // too many arguments
	Yaml(serde_yaml::Error),
	Io(io::Error), // no file, invalid perm...
}

impl fmt::Display for TaskmasterError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			TaskmasterError::ParsingError(msg) => write!(f, "Parsing error : {}", msg),
			TaskmasterError::InvalidParam(msg) => write!(f, "Invalid param : {}", msg),
			TaskmasterError::Argument(msg) => write!(f, "Argument error : {}", msg),
			TaskmasterError::Yaml(err) => write!(f, "Error YAML : {}", err),
			TaskmasterError::Io(err) => write!(f, "Erreur système I/O : {}", err),
		}
	}
}

impl Error for TaskmasterError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			TaskmasterError::ParsingError(_) => None,
			TaskmasterError::InvalidParam(_) => None,
			TaskmasterError::Argument(_) => None,
			TaskmasterError::Io(err) => Some(err),
			TaskmasterError::Yaml(err) => Some(err),
		}
	}
}

impl From<io::Error> for TaskmasterError {
	fn from(err: io::Error) -> Self {
		TaskmasterError::Io(err)
	}
}

impl From<serde_yaml::Error> for TaskmasterError {
	fn from(err: serde_yaml::Error) -> Self {
		TaskmasterError::Yaml(err)
	}
}
