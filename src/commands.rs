use calib_error::CalibError;
use co_no2_kombisensor::kombisensor::Kombisensor;
use libmodbus_rs::modbus::Modbus;
use std::sync::{Arc, Mutex};

type Result<T> = ::std::result::Result<T, CalibError>;


/// Speichert ein Wert in einem Register
///
pub fn sensor_new_adc_at_nullgas(kombisensor: &Arc<Mutex<Kombisensor>>, adc_value: i32) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    let slave_id: u8 = kombisensor.get_modbus_address();
    let register_address: i32 = 14; // Im Modbus Register[3] ist die Modbus Adresse gespeichert.

    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.connect());
    try!(modbus.write_register(register_address, adc_value));

    Ok(())
}


/// Speichert eine neue Modbus Adresse im Kombisensor
///
pub fn kombisensor_new_modbus_address(kombisensor: &Arc<Mutex<Kombisensor>>, new_modbus_address: i32) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    let slave_id: u8 = kombisensor.get_modbus_address();
    let register_address: i32 = 3; // Im Modbus Register[3] ist die Modbus Adresse gespeichert.

    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.connect());
    try!(modbus.write_register(register_address, new_modbus_address));

    Ok(())
}

/// Modbus Neustart Befehl
///
/// Diese Funktion sendet ein raw request zu dem Server, der Modbus Funktion Code 0x08,
/// Sub-Funktion Code 0x01 startet die Sensor Hardware neu.
///
pub fn kombisensor_restart_via_modbus(kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));
    let slave_id: u8 = kombisensor.get_modbus_address();
    let raw_request = vec![slave_id, 0x08, 0x0, 0x01];

    try!(modbus.connect());
    try!(modbus.send_raw_request(&raw_request));

    Ok(())
}

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

/// Kombisensor Daten in der Kombisensor Hardware speichern
///
pub fn kombisensor_to_modbus(kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    #[cfg(debug_assertions)] // cfg(debug_assertions) sorgt dafür,
    // dass die Modbus Debug Nachrichten nicht in release Builds ausgegeben werden.
    try!(modbus.set_debug(true));
    let slave_id: u8 = kombisensor.get_modbus_address();

    try!(modbus.connect());

    Ok(())
}

/// Aktiviere/ Deaktiviere eine Sensor Messzelle auf der Hardware
///
/// Diese Funktion sendet entweder das Modbus Coil 0 (NO2 Sensor) oder das Coil 16 (CO Sensor).
/// Der Coil status (an/ aus) wird mit dem Parameter sensor_state übergeben.
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
        "CO" =>  try!(modbus.write_bit(0x16, sensor_state)),
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
