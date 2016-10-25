use libmodbus_rs::modbus::{Error, Modbus};

type Result<T> = ::std::result::Result<T, Error>;

/// Aktiviert/ Deaktiviert den NO2 Sensor
///
/// # Attributes
/// `state`     - boolen, true to enable sensor and false to disable the sensor
///
pub fn enable_no2(state: bool) -> Result<()> {
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(200));
    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));

    match modbus.connect() {
        Err(err) => {
            modbus.free();
            // Return Error
            return Err(err);
        }
        Ok(_) => {
            modbus.write_bit(0x00, state);
        }
    }
    Ok(())
}

/// Aktiviert/ Deaktiviert den CO Sensor
///
/// # Attributes
/// `state`     - boolen, true to enable sensor and false to disable the sensor
///
pub fn enable_co(state: bool) -> Result<()> {
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(200));
    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));

    match modbus.connect() {
        Err(err) => {
            modbus.free();
            // Return Error
            return Err(err);
        }
        Ok(_) => {
            modbus.write_bit(0x01, state);
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use ::libmodbus_rs::modbus::Error::ConnectionError;

    #[test]
    fn test_enable_no2() {
        let result = enable_no2(true);
        assert_eq!(result, Err(ConnectionError));
    }
    #[test]
    fn test_enable_co() {
        let result = enable_no2(true);
        assert_eq!(result, Err(ConnectionError));
    }
}
