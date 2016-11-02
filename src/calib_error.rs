use std::convert::From;
use libmodbus_rs;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CalibError {
    SerialInterfaceUnknown,
    Modbus(libmodbus_rs::Error),
}

impl fmt::Display for CalibError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CalibError::SerialInterfaceUnknown => write!(f, "Serielle Schnittstelle ist nicht bekannt."),
            CalibError::Modbus(ref err) => err.fmt(f),
        }
    }
}

impl Error for CalibError {
    fn description(&self) -> &str {
        match *self {
            CalibError::SerialInterfaceUnknown => "Unbekannte serielle Schnittstelle",
            CalibError::Modbus(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CalibError::SerialInterfaceUnknown => None,
            CalibError::Modbus(ref err) => Some(err),
        }
    }
}

impl From<libmodbus_rs::Error> for CalibError {
    fn from(err: libmodbus_rs::Error) -> CalibError {
        CalibError::Modbus(err)
    }
}
