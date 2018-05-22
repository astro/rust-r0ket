use table::table;

pub struct Voltage(pub u32);

impl Voltage {
    pub fn is_critical(&self) -> bool {
        self.0 < 3550
    }

    pub fn is_low(&self) -> bool {
        self.0 < 3650
    }

    pub fn is_ok(&self) -> bool {
        self.0 >= 3650
    }

    pub fn is_full(&self) -> bool {
        self.0 >= 4120
    }
}

pub fn get_voltage() -> Voltage {
    Voltage((table().GetVoltage)())
}
