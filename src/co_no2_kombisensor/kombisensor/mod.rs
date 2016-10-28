
pub struct Kombisensor {
    version: u32,
    pub modbus_address: u8,
}

impl Kombisensor {
    pub fn new() -> Self {
        Kombisensor {
            version: 0,
            modbus_address: 0,
        }
    }
}
