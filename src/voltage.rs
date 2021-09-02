#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Voltage {
    millivolts: i16,
}

impl Voltage {
    pub const fn from_millivolts(mv: i16) -> Self {
        Self { millivolts: mv }
    }

    pub const fn from_volts(v: i16) -> Self {
        Self::from_millivolts(v * 1000)
    }

    pub const fn as_millivolts(self) -> i16 {
        self.millivolts
    }
}
