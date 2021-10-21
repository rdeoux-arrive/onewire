use crate::Voltage;
use core::time::Duration;

pub struct Settings {
    pub period: Duration,
    pub voltage: Voltage,
    pub offset: i32,
}

impl Settings {
    const PERIODS: [Duration; 10] = [
        Duration::from_micros(100),
        Duration::from_micros(200),
        Duration::from_micros(500),
        Duration::from_millis(1),
        Duration::from_millis(2),
        Duration::from_millis(5),
        Duration::from_millis(10),
        Duration::from_millis(20),
        Duration::from_millis(50),
        Duration::from_millis(100),
    ];

    pub fn inc_period(&mut self) {
        self.period = *Self::PERIODS
            .iter()
            .find(|per| *per > &self.period)
            .unwrap_or(&self.period);
    }

    pub fn dec_period(&mut self) {
        self.period = *Self::PERIODS
            .iter()
            .rev()
            .find(|per| *per < &self.period)
            .unwrap_or(&self.period);
    }

    const VOLTAGES: [Voltage; 3] = [
        Voltage::from_volts(1),
        Voltage::from_volts(2),
        Voltage::from_volts(5),
    ];

    pub fn inc_voltage(&mut self) {
        self.voltage = *Self::VOLTAGES
            .iter()
            .find(|per| *per > &self.voltage)
            .unwrap_or(&self.voltage);
    }

    pub fn dec_voltage(&mut self) {
        self.voltage = *Self::VOLTAGES
            .iter()
            .rev()
            .find(|per| *per < &self.voltage)
            .unwrap_or(&self.voltage);
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            period: Self::PERIODS[0],
            voltage: Self::VOLTAGES[0],
            offset: 0,
        }
    }
}
