use libmodbus_rs::modbus::{Error, Modbus};
use co_no2_kombisensor::kombisensor::Kombisensor;
use std::sync::{Arc, Weak};
use std::borrow::{Borrow, BorrowMut};

type Result<T> = ::std::result::Result<T, Error>;


pub fn kombisensor_from_modbus(kombisensor: Weak<Kombisensor>) -> Result<()> {
    // Mache wieder ein Kombisensor aus dem weak pointer
    // let kombisensor = Borrow::<Kombisensor>::borrow(&kombisensor.upgrade().unwrap());
    // let mut kombisensor = Borrow::<Kombisensor>::borrow(&kombisensor.upgrade().unwrap());
    // kombisensor.modbus_address = 1;

    Ok(())
}

pub fn enable_sensor(sensor_type: &str, sensor_state: bool) -> Result<()> {
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(200));
    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));

    match modbus.connect() {
        Err(err) => return Err(err),
        Ok(_) => {
            match sensor_type {
                "NO2" => { try!(modbus.write_bit(0x00, sensor_state)); }
                "CO" => { try!(modbus.write_bit(0x01, sensor_state)); }
                _ => {}
            }
        }
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // Solange wir kein richtiges Mock oder wenigstens TCP/IP Modbus Haben
    fn test_enable_no2() {
        let result = enable_sensor("NO2", true);
        assert_eq!(result, Ok(()));
    }
    #[test]
    #[ignore] // Solange wir kein richtiges Mock oder wenigstens TCP/IP Modbus Haben
    fn test_enable_co() {
        let result = enable_sensor("CO", true);
        assert_eq!(result, Ok(()));
    }
}
