use std::fmt::Result;
use std::fmt::Display;
use std::fmt::Formatter;
use std::error::Error;

#[derive(Debug)]
pub enum CommandError {
    ExitedWithError,
    EnvNotFound(String),
    InvalidCommand(String),
    NoMachinesFound,
    InvalidNumber(u16),
    MachineNotBootable,
}

impl Into<String> for CommandError {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl Display for CommandError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match *self {
            CommandError::ExitedWithError => write!(f, "Command exited with errors"),
            CommandError::EnvNotFound(ref name) => {
                write!(f,
                       "The environment variable '{}' does not appear to be set",
                       name)
            }
            CommandError::InvalidCommand(ref name) => write!(f, "Invalid command '{}'", name),
            CommandError::NoMachinesFound => {
                write!(f, "Could not find any vagrant machines available")
            }
            CommandError::InvalidNumber(ref number) => {
                write!(f, "Invalid machine number '{}'", number)
            }
            CommandError::MachineNotBootable => write!(f, "Could not boot up the machine"),
        }
    }
}

#[allow(unused_variables)]
impl Error for CommandError {
    fn description(&self) -> &str {
        match *self {
            CommandError::ExitedWithError => "This error occurs when a command exited with errors",
            CommandError::EnvNotFound(ref name) => {
                "This error occurs when environment variable has not been set"
            }
            CommandError::InvalidCommand(ref name) => {
                "This error occurs when a command is not recognized"
            }
            CommandError::NoMachinesFound => "This error occurs when no machines were found",
            CommandError::InvalidNumber(ref number) => {
                "This error occurs when a number does not match any machine"
            }
            CommandError::MachineNotBootable => {
                "This error occurs when the machine could not be booted up"
            }
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            _ => None,
        }
    }
}
