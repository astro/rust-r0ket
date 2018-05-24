use table::table;

/// Wraps millivolts
pub struct Voltage(pub u32);

impl Voltage {
    /// Battery voltage critically low?
    pub fn is_critical(&self) -> bool {
        self.0 < 3550
    }

    /// Battery low?
    pub fn is_low(&self) -> bool {
        self.0 < 3650
    }

    /// Battery not low?
    pub fn is_ok(&self) -> bool {
        self.0 >= 3650
    }

    /// Battery fully charged?
    pub fn is_full(&self) -> bool {
        self.0 >= 4120
    }
}

/// Get current battery voltage
pub fn get_voltage() -> Voltage {
    Voltage((table().GetVoltage)())
}
