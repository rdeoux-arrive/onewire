#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Voltage {
    millivolts: i16,
}

impl Voltage {
    #[must_use]
    pub const fn from_millivolts(mv: i16) -> Self {
        Self { millivolts: mv }
    }

    #[must_use]
    pub const fn from_volts(v: i16) -> Self {
        Self::from_millivolts(v * 1000)
    }

    #[must_use]
    pub const fn as_millivolts(self) -> i16 {
        self.millivolts
    }
}
