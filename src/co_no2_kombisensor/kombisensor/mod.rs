use co_no2_kombisensor::sensor::{Sensor, SensorType};
use std::str::FromStr;

#[derive(Debug)]
pub struct Kombisensor {
    version: String,
    modbus_address: u8,
    pub sensors: Vec<Sensor>,
    live_update: bool,
}

impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {
            version: "0.0.0".to_string(),
            modbus_address: 247,
            sensors: vec![Sensor::new(SensorType::RaGasNO2), Sensor::new(SensorType::RaGasCO)],
            live_update: false,
        }
    }

// GETTER
    /// Liefert die Versionsnummer
    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    /// Liefert die Modbus Adresse
    pub fn get_modbus_address(&self) -> u8 {
        self.modbus_address
    }

    /// Status des live_update Flags
    pub fn get_live_update(&self) -> bool {
        self.live_update
    }

// SETTER
    /// Setzt die Versionsnummer
    pub fn set_version(&mut self, version: String) {
        self.version = version;
    }

    /// Setzt die Modbus Adresse
    pub fn set_modbus_address(&mut self, modbus_address: u8) {
        self.modbus_address = modbus_address
    }

    /// Status des live_update Flags
    pub fn set_live_update(&mut self, live_update: bool) {
        self.live_update = live_update
    }

//MISC
    /// Parsed den übergebenen Vector aus Bytes und füllt die Member der eigenen Datenstruktur
    ///
    pub fn parse_modbus_registers(&mut self, modbus_registers: Vec<u16>) {
        let version = format!("{}.{}.{}", modbus_registers[0], modbus_registers[1], modbus_registers[2]);
        let sensor1_enabled = (modbus_registers[18] >> 1) & 1;
        let sensor2_enabled = (modbus_registers[28] >> 1) & 1;

        self.set_version(version);

        if sensor1_enabled == 0 {
            self.sensors[0].set_number(modbus_registers[10]);
            self.sensors[0].set_adc_value(modbus_registers[11]);
            self.sensors[0].set_min_value(modbus_registers[12]);
            self.sensors[0].set_max_value(modbus_registers[13]);
            self.sensors[0].set_adc_at_nullgas(modbus_registers[14]);
            self.sensors[0].set_adc_at_messgas(modbus_registers[15]);
            self.sensors[0].set_concentration_at_nullgas(modbus_registers[16]);
            self.sensors[0].set_concentration_at_messgas(modbus_registers[17]);
            self.sensors[0].set_config(modbus_registers[18]);
        }

        if sensor2_enabled == 0 {
            self.sensors[1].set_number(modbus_registers[20]);
            self.sensors[1].set_adc_value(modbus_registers[21]);
            self.sensors[1].set_min_value(modbus_registers[22]);
            self.sensors[1].set_max_value(modbus_registers[23]);
            self.sensors[1].set_adc_at_nullgas(modbus_registers[24]);
            self.sensors[1].set_adc_at_messgas(modbus_registers[25]);
            self.sensors[1].set_concentration_at_nullgas(modbus_registers[26]);
            self.sensors[1].set_concentration_at_messgas(modbus_registers[27]);
            self.sensors[1].set_config(modbus_registers[28]);
        }
    }

    /// Exportiert die eigene Datenstruktur in die Modbus Datenstruktur
    ///
    /// # Examples
    ///
    /// ```
    /// use kalibrator::*;
    ///
    /// let mut kombisensor = Kombisensor::new();
    /// kombisensor.set_version("1.2.3".to_string());
    ///
    /// let modbus_registers = kombisensor.to_modbus_registers();
    /// assert_eq!(modbus_registers[0], 1);
    /// assert_eq!(modbus_registers[1], 2);
    /// assert_eq!(modbus_registers[2], 3);
    ///
    /// assert_eq!(modbus_registers[3], 247);
    /// ```
    ///
    #[allow(dead_code)]
    pub fn to_modbus_registers(&self) -> Vec<u16> {
        let mut modbus_registers: Vec<u16> = vec![0u16; 30];

        let vec_version: Vec<u16> = self.version.split(".").map(|s| {
            u16::from_str(s).unwrap_or(0u16)
        }).collect();

        modbus_registers[0] = vec_version[0];
        modbus_registers[1] = vec_version[1];
        modbus_registers[2] = vec_version[2];
        modbus_registers[3] = self.modbus_address as u16;

        modbus_registers[10] = self.sensors[0].get_number();
        modbus_registers[11] = self.sensors[0].get_adc_value();
        modbus_registers[12] = self.sensors[0].get_min_value();
        modbus_registers[13] = self.sensors[0].get_max_value();
        modbus_registers[14] = self.sensors[0].get_adc_at_nullgas();
        modbus_registers[15] = self.sensors[0].get_adc_at_messgas();
        modbus_registers[16] = self.sensors[0].get_concentration_at_nullgas();
        modbus_registers[17] = self.sensors[0].get_concentration_at_messgas();

        modbus_registers[20] = self.sensors[1].get_number();
        modbus_registers[21] = self.sensors[1].get_adc_value();
        modbus_registers[22] = self.sensors[1].get_min_value();
        modbus_registers[23] = self.sensors[1].get_max_value();
        modbus_registers[24] = self.sensors[1].get_adc_at_nullgas();
        modbus_registers[25] = self.sensors[1].get_adc_at_messgas();
        modbus_registers[26] = self.sensors[1].get_concentration_at_nullgas();
        modbus_registers[27] = self.sensors[1].get_concentration_at_messgas();

        modbus_registers
    }
}
