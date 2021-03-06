use calib_error::CalibError;
use co_no2_kombisensor::kombisensor::Kombisensor;
use co_no2_kombisensor::sensor::SensorType;
use gas::GasType;
use libmodbus_rs::modbus::Modbus;
use std::sync::{Arc, Mutex};

type Result<T> = ::std::result::Result<T, CalibError>;


/// Speichert den ADC Wert eines Sensors für Null.- oder Messgas
///
#[allow(unused_assignments)]
pub fn sensor_new_adc_at(gas_type: &GasType, sensor_type: &SensorType, kombisensor: &Arc<Mutex<Kombisensor>>, adc_value: i32) -> Result<()> {
    let kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);

    let sensor_num: usize = match *sensor_type {
        SensorType::RaGasNO2 => 0,
        SensorType::RaGasCO => 1,
    };
    let register_address_offset = match *gas_type {
        GasType::Nullgas => { 14 }  // Register 14 oder 24 ADC Nullgas
        GasType::Messgas => { 15 }  // Register 15 oder 25 ADC Nullgas
    };
    let register_address: i32 = (sensor_num as i32 * 10) + register_address_offset;

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
    let register_address: i32 = 3; // Im Modbus Register[3] ist die Modbus Adresse gespeichert.

    try!(modbus.set_debug(true));
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.connect());
    try!(modbus.write_register(register_address, new_modbus_address));

    // Aktualisiere die Kombisensor Instanc im RAM
    kombisensor.set_modbus_address(new_modbus_address as u8);

    Ok(())
}

/// Modbus Neustart Befehl
///
/// Diese Funktion sendet ein raw request zu dem Server, der Modbus Funktion Code 0x08,
/// Sub-Funktion Code 0x01 startet die Sensor Hardware neu.
///
#[allow(dead_code)]
pub fn kombisensor_restart_via_modbus(kombisensor: &Arc<Mutex<Kombisensor>>) -> Result<()> {
    let kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
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
    try!(modbus.set_debug(true));

    try!(modbus.connect());
    try!(modbus.read_registers(0, 30).map(|registers| {
        kombisensor.parse_modbus_registers(registers);
    }));

    Ok(())
}

/// Kombisensor Daten in der Kombisensor Hardware speichern
///
#[allow(dead_code)]
pub fn kombisensor_to_modbus(kombisensor: &Arc<Mutex<Kombisensor>>, values: &Vec<u16>) -> Result<()> {
    let kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);

    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.set_debug(true));

    try!(modbus.connect());
    try!(modbus.write_registers(0, values));

    Ok(())
}

/// Aktiviere/ Deaktiviere eine Sensor Messzelle auf der Hardware
///
/// Diese Funktion sendet entweder das Modbus Coil 0 (NO2 Sensor) oder das Coil 16 (CO Sensor).
/// Der Coil status (an/ aus) wird mit dem Parameter sensor_state übergeben.
pub fn enable_sensor(kombisensor: &Arc<Mutex<Kombisensor>>, sensor_type: SensorType, sensor_state: bool) -> Result<()> {
    let kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.set_debug(true));
    try!(modbus.connect());

    match sensor_type {
        SensorType::RaGasNO2 => {try!(modbus.write_bit(00, sensor_state))}
        SensorType::RaGasCO =>  {try!(modbus.write_bit(16, sensor_state))}
    }

    Ok(())
}

/// Speichere Min Value des Sensors
///
pub fn sensor_save_min(kombisensor: &Arc<Mutex<Kombisensor>>, sensor_type: &SensorType, min_value: i32) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.set_debug(true));
    try!(modbus.connect());

    match *sensor_type {
        SensorType::RaGasNO2 => {
            try!(modbus.write_register(12, min_value));
            kombisensor.sensors[0].set_min_value(min_value as u16);
        }
        SensorType::RaGasCO =>  {
            try!(modbus.write_register(22, min_value));
            kombisensor.sensors[1].set_min_value(min_value as u16);
        }
    }

    Ok(())
}

/// Speichere Max Value des Sensors
///
pub fn sensor_save_max(kombisensor: &Arc<Mutex<Kombisensor>>, sensor_type: &SensorType, max_value: i32) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.set_debug(true));
    try!(modbus.connect());

    match *sensor_type {
        SensorType::RaGasNO2 => {
            try!(modbus.write_register(13, max_value));
            kombisensor.sensors[0].set_max_value(max_value as u16);
        }
        SensorType::RaGasCO =>  {
            try!(modbus.write_register(23, max_value));
            kombisensor.sensors[1].set_max_value(max_value as u16);
        }
    }

    Ok(())
}

/// Speichere Konzentration Nullgas
///
pub fn sensor_save_conz_nullgas(kombisensor: &Arc<Mutex<Kombisensor>>, sensor_type: &SensorType, conz_nullgas_value: i32) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.set_debug(true));
    try!(modbus.connect());

    match *sensor_type {
        SensorType::RaGasNO2 => {
            try!(modbus.write_register(16, conz_nullgas_value));
            kombisensor.sensors[0].set_concentration_at_nullgas(conz_nullgas_value as u16);
        }
        SensorType::RaGasCO =>  {
            try!(modbus.write_register(26, conz_nullgas_value));
            kombisensor.sensors[1].set_concentration_at_nullgas(conz_nullgas_value as u16);
        }
    }

    Ok(())
}

/// Speichere Konzentration Messgas
///
pub fn sensor_save_conz_messgas(kombisensor: &Arc<Mutex<Kombisensor>>, sensor_type: &SensorType, conz_messgas_value: i32) -> Result<()> {
    let mut kombisensor = kombisensor.lock().unwrap();
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    try!(modbus.set_slave(kombisensor.get_modbus_address() as i32));
    try!(modbus.set_debug(true));
    try!(modbus.connect());

    match *sensor_type {
        SensorType::RaGasNO2 => {
            try!(modbus.write_register(17, conz_messgas_value));
            kombisensor.sensors[0].set_concentration_at_messgas(conz_messgas_value as u16);
        }
        SensorType::RaGasCO =>  {
            try!(modbus.write_register(27, conz_messgas_value));
            kombisensor.sensors[1].set_concentration_at_messgas(conz_messgas_value as u16);
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
        // let result = enable_sensor("NO2", true);
        // assert_eq!(result, Ok(()));
    }
    #[test]
    #[ignore] // Solange wir kein richtiges Mock oder wenigstens TCP/IP Modbus Haben
    fn test_enable_co() {
        // let result = enable_sensor("CO", true);
        // assert_eq!(result, Ok(()));
    }
}
