/// Aktuelle Rust Representation des CO-NO2 Kombisensors (Firmware Version: 0.11.1)

#[derive(Debug)]
pub enum SensorType {
    RaGasNO2,
    RaGasCO,
}

#[derive(Debug)]
pub enum SI {
    none,
    ppm,
    UEG,
    Vol,
}

#[derive(Debug)]
pub struct Sensor {
    number: u16,
    adc_value: u16,
    min_value: u16,
    max_value: u16,
    adc_at_nullgas: u16,
    adc_at_messgas: u16,
    concentration_nullgas: u16,
    concentration_messgas: u16,
    sensor_type: SensorType, // Nicht direkt in der Kombisensor Firmware (coil 0 für Sensor1 und 16 für Sensor2)
    si: SI, // Nicht direkt in der Kombisensor Firmware/ Modbus Datenstruktur (coils 1..3 für Sensor1 usw.)
}

impl Sensor {
    pub fn new(sensor_type: SensorType) -> Self {
        Sensor {
            number: 0,
            adc_value: 0,
            min_value: 0,
            max_value: 0,
            adc_at_nullgas: 0,
            adc_at_messgas: 0,
            concentration_nullgas: 0,
            concentration_messgas: 0,
            sensor_type: sensor_type,
            si: SI::ppm,
        }
    }

// SETTER
    pub fn set_number(&mut self, number: u16) {
        self.number = number;
    }

    pub fn set_adc_value(&mut self, adc_value: u16) {
        self.adc_value = adc_value;
    }

    pub fn set_min_value(&mut self, min_value: u16) {
        self.min_value = min_value;
    }

    pub fn set_max_value(&mut self, max_value: u16) {
        self.max_value = max_value;
    }

    pub fn set_adc_at_nullgas(&mut self, adc_at_nullgas: u16) {
        self.adc_at_nullgas = adc_at_nullgas;
    }

    pub fn set_adc_at_messgas(&mut self, adc_at_messgas: u16) {
        self.adc_at_messgas = adc_at_messgas;
    }

    pub fn set_concentration_nullgas(&mut self, concentration_nullgas: u16) {
        self.concentration_nullgas = concentration_nullgas;
    }

    pub fn set_concentration_messgas(&mut self, concentration_messgas: u16) {
        self.concentration_messgas = concentration_messgas;
    }
// GETTER
    pub fn get_number(&self) -> u16 {
        self.number
    }

    pub fn get_adc_value(&self) -> u16 {
        self.adc_value
    }

    pub fn get_min_value(&self) -> u16 {
        self.min_value
    }

    pub fn get_max_value(&self) -> u16 {
        self.max_value
    }

    pub fn get_adc_at_nullgas(&self) -> u16 {
        self.adc_at_nullgas
    }

    pub fn get_adc_at_messgas(&self) -> u16 {
        self.adc_at_messgas
    }

    pub fn get_concentration_nullgas(&self) -> u16 {
        self.concentration_nullgas
    }

    pub fn get_concentration_messgas(&self) -> u16 {
        self.concentration_messgas
    }
/// MISC
    pub fn get_mv(&self) -> u16 {
        (5000 / 1024) * self.adc_value as u16
    }

    pub fn get_value(&self) -> u16 {
        let adc_value = self.adc_value;
        let concentration_nullgas = self.concentration_nullgas;
        let concentration_messgas = self.concentration_messgas;
        let adc_at_nullgas = self.adc_at_nullgas;
        // Damit wir in der Formel nicht durch Null teilen, wird der Wert adc_at_messgas auf 1 gesetzt, sollte er Null sein
        let adc_at_messgas = if self.adc_at_messgas == 0 {1} else {self.adc_at_messgas};

        println!("({} - {}) / ({} - {}) * ({} - {}) + {}", concentration_messgas, concentration_nullgas,
        adc_at_messgas, adc_at_nullgas,
        adc_value, adc_at_nullgas, concentration_nullgas);
        
        ((concentration_messgas - concentration_nullgas) /
        (adc_at_messgas - adc_at_nullgas)) *
        (adc_value - adc_at_nullgas) + concentration_nullgas
        // 0
    }

}
