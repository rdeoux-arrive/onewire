use crate::voltage::Voltage;
use core::convert::TryFrom;
use core::num::ParseFloatError;
use csv::StringRecord;

#[derive(Debug)]
pub enum Error {
    MissingRow,
    TooManyRows,
    ParseFloat(ParseFloatError),
    OutOfBoundsTimestamp(f32),
    OutOfBoundsVoltage(f32),
}

#[derive(Default)]
pub struct Sample {
    pub timestamp: i32,
    pub voltage: Voltage,
}

impl TryFrom<StringRecord> for Sample {
    type Error = Error;

    fn try_from(record: StringRecord) -> Result<Self, Self::Error> {
        let mut cells = record.into_iter();
        let timestamp_s: f32 = cells
            .next()
            .ok_or(Error::MissingRow)?
            .parse()
            .map_err(Error::ParseFloat)?;
        let timestamp_us = timestamp_s * 1e6;
        if timestamp_us < i32::MIN as f32 || timestamp_us > i32::MAX as f32 {
            return Err(Error::OutOfBoundsTimestamp(timestamp_us));
        }

        let voltage_v: f32 = cells
            .next()
            .ok_or(Error::MissingRow)?
            .parse()
            .map_err(Error::ParseFloat)?;
        let voltage_mv = voltage_v * 1e3;
        if voltage_mv < f32::from(i16::MIN) || voltage_mv > f32::from(i16::MAX) {
            return Err(Error::OutOfBoundsVoltage(voltage_mv));
        }

        if cells.next().is_some() {
            return Err(Error::TooManyRows);
        }

        Ok(Self {
            timestamp: timestamp_us as i32,
            voltage: Voltage::from_millivolts(voltage_mv as i16),
        })
    }
}
