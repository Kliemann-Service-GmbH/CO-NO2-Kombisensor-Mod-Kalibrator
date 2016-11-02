
#[derive(Debug)]
pub enum SensorType {
    RaGasNO2,
    RaGasCO,
}

#[derive(Debug)]
pub struct Sensor {
    sensor_type: SensorType,
    number: u16,
    adc_value: u16,
    min_value: u16,
    max_value: u16,
    adc_at_nullgas: u16,
    adc_at_messgas: u16,
    si: u16,
}

impl Sensor {
    pub fn new(sensor_type: SensorType) -> Self {
        Sensor {
            sensor_type: sensor_type,
            number: 0,
            adc_value: 0,
            min_value: 0,
            max_value: 0,
            adc_at_nullgas: 0,
            adc_at_messgas: 0,
            si: 0,
        }
    }

// GETTER
    /// Liefert den ADC Wert
    pub fn get_adc_value(&self) -> u16 {
        self.adc_value
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

    pub fn set_si(&mut self, si: u16) {
        self.si = si;
    }
}
