use calib_error::CalibError;
use std::path::Path;


pub struct Configuration {
    serial_interface: &'static str,
}

impl Configuration {
    pub fn new() -> Self {
        Configuration {
            serial_interface: "/dev/ttyUSB0",
            // serial_interface: "/dev/ttyS1",
        }
    }

    pub fn get_serial_interface(&self) -> &'static str {
        self.serial_interface
    }

    pub fn set_serial_interface(&mut self, interface: &'static str) {
        self.serial_interface  = interface
    }

    pub fn is_valid(&self) -> Result<(), CalibError> {
        if Path::new(self.serial_interface).exists() {
            Ok(())
        } else {
            Err(CalibError::SerialInterfaceUnknown)
        }
    }
}
