use calib_error::CalibError;
use co_no2_kombisensor::kombisensor::Kombisensor;
use libmodbus_rs::modbus::Modbus;
// use std::borrow::{Borrow, BorrowMut};
use std::sync::{Arc, Mutex};

type Result<T> = ::std::result::Result<T, CalibError>;


/// Modbus Datenbus nach dem ersten verfügbaren Sensor scannen.
///
/// Diese Funktion beginnt Ihre Suche bei der Modbus Adresse 247, default Adresse der RA-Gas Sensoren.
/// Der Funktion wird ein Pointer zur Kombisensor Datenstruktur übergeben, wird ein Sensor erkannt
/// dann wird der modbus_address Memeber der Struct gesetzt.
///
pub fn kombisensor_discovery(kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));

    // Erlaubte Modbus Adressen von 1-247 durchsuchen, in umgekehrter Reihenfolge, beginnend bei 247.
    for i in (1..248).rev() {
        try!(modbus.set_slave(i));
        try!(modbus.connect());

        match modbus.read_registers(0, 1) {
            Ok(_) => {
                kombisensor.set_modbus_address(i as u8);
                break;
            }
            Err(_) => {}, // Im Fehlerfall nichts machen
        }
    }
    Ok(())
}


/// Kombisensor Datenstruktur füllen
///
/// Diese Funktion liest alle Register des Kombisensors aus und speichert die Werte in der
/// Kombisensor Datenstruktur.
///
pub fn kombisensor_from_modbus(kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<()> {
    use std::mem;

    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);

    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));

    try!(modbus.connect());
    try!(modbus.read_registers(0, 30).map(|registers| {
        kombisensor.parse_modbus_registers(registers);
    }));

    Ok(())
}

pub fn enable_sensor(kombisensor: &Arc<Mutex<Kombisensor>>, sensor_type: &str, sensor_state: bool) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));
    try!(modbus.connect());

    match sensor_type {
        "NO2" => try!(modbus.write_bit(0x00, sensor_state)),
        "CO" =>  try!(modbus.write_bit(0x01, sensor_state)),
        _ => {}
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
