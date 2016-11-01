use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CalibError {
    SerialInterfaceUnknown,
}

impl fmt::Display for CalibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalibError::SerialInterfaceUnknown => write!(f, "Serielle Schnittstelle ist nicht bekannt."),
        }
    }
}

impl Error for CalibError {
    fn description(&self) -> &str {
        match *self {
            CalibError::SerialInterfaceUnknown => "Unbekannte serielle Schnittstelle",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CalibError::SerialInterfaceUnknown => None,
        }
    }
}
