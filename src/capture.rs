use core::{iter::Peekable, ops::Deref, time::Duration};
use std::io::Read;

use csv::{self, ReaderBuilder};

use crate::{Sample, Voltage};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Csv(#[from] csv::Error),
    #[error("missing header")]
    MissingHeader,
    #[error("{0}")]
    InvalidRecord(#[from] crate::sample::Error),
}

#[derive(Default)]
pub struct Capture {
    samples: Vec<Sample>,
}

impl Capture {
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let reader = ReaderBuilder::new().has_headers(false).from_reader(reader);
        let mut records = reader.into_records();

        for headers in &[&["x-axis", "1"] as &[&str], &["second", "Volt"]] {
            let record = records.next().ok_or(Error::MissingHeader)??;
            if record != **headers {
                return Err(Error::MissingHeader);
            }
        }

        let mut samples = Vec::new();
        for record in records {
            samples.push(record?.try_into()?);
        }

        Ok(Self { samples })
    }

    pub fn edges(&self) -> Edges<core::slice::Iter<'_, Sample>> {
        Edges {
            state: EdgeState::Undetermined,
            samples: self.iter(),
        }
    }

    pub fn slots(&self) -> Slots<'_, core::slice::Iter<'_, Sample>> {
        Slots {
            edges: self.edges().peekable(),
        }
    }

    pub fn bytes(&self) -> Bytes<'_, core::slice::Iter<'_, Sample>> {
        Bytes {
            slots: self.slots(),
            byte: None,
            bits: 0,
        }
    }
}

impl Deref for Capture {
    type Target = [Sample];

    fn deref(&self) -> &Self::Target {
        &self.samples
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Edge {
    Fall { timestamp: i32 },
    Raise { timestamp: i32 },
}

enum EdgeState {
    Undetermined,
    High,
    Low,
}

pub struct Edges<T> {
    state: EdgeState,
    samples: T,
}

impl<'a, T: Iterator<Item = &'a Sample>> Iterator for Edges<T> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        const V_TL: Voltage = Voltage::from_millivolts(500);
        const V_TH: Voltage = Voltage::from_volts(1);

        for sample in self.samples.by_ref() {
            let timestamp = sample.timestamp;
            match self.state {
                EdgeState::Undetermined => {
                    if sample.voltage > V_TH {
                        self.state = EdgeState::High;
                    } else if sample.voltage < V_TL {
                        self.state = EdgeState::Low;
                    }
                }
                EdgeState::High => {
                    if sample.voltage < V_TL {
                        self.state = EdgeState::Low;
                        return Some(Edge::Fall { timestamp });
                    }
                }
                EdgeState::Low => {
                    if sample.voltage > V_TH {
                        self.state = EdgeState::High;
                        return Some(Edge::Raise { timestamp });
                    }
                }
            }
        }

        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotKind {
    Reset,
    One,
    WriteZero,
    ReadZero,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Slot {
    pub kind: SlotKind,
    pub timestamp: i32,
    pub duration: Duration,
}

pub struct Slots<'a, T>
where
    T: Iterator<Item = &'a Sample>,
{
    edges: Peekable<Edges<T>>,
}

impl<'a, T> Iterator for Slots<'a, T>
where
    T: Iterator<Item = &'a Sample>,
{
    type Item = Slot;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(edge) = self.edges.next() {
            if let Edge::Fall { timestamp: t0 } = edge
                && let Some(Edge::Raise { timestamp: t1 }) = self.edges.next()
            {
                if (480..=640).contains(&(t1 - t0)) {
                    if let Some(Edge::Fall { timestamp: t2 }) = self.edges.next()
                        && (15..=60).contains(&(t2 - t1))
                        && let Some(Edge::Raise { timestamp: t3 }) = self.edges.next()
                        && (60..=240).contains(&(t3 - t2))
                        && let Some(Edge::Fall { timestamp: t4 }) = self.edges.peek()
                        && t4 - t3 > 5
                    {
                        return Some(Slot {
                            kind: SlotKind::Reset,
                            timestamp: t0,
                            duration: Duration::from_millis((t1 + 305 - t0).try_into().unwrap()),
                        });
                    }
                } else if (60..=120).contains(&(t1 - t0)) {
                    if let Some(Edge::Fall { timestamp: t2 }) = self.edges.peek()
                        && t2 - t1 >= 5
                    {
                        return Some(Slot {
                            kind: SlotKind::WriteZero,
                            timestamp: t0,
                            duration: Duration::from_millis((t1 - t0 + 5).try_into().unwrap()),
                        });
                    }
                } else if (15..=60).contains(&(t1 - t0)) {
                    if let Some(Edge::Fall { timestamp: t2 }) = self.edges.peek()
                        && t2 - t1 >= 5
                    {
                        return Some(Slot {
                            kind: SlotKind::ReadZero,
                            timestamp: t0,
                            duration: Duration::from_millis(65),
                        });
                    }
                } else if (5..=15).contains(&(t1 - t0))
                    && let Some(Edge::Fall { timestamp: t2 }) = self.edges.peek()
                    && t2 - t0 >= 65
                {
                    return Some(Slot {
                        kind: SlotKind::One,
                        timestamp: t0,
                        duration: Duration::from_millis(65),
                    });
                }
            }
        }

        None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Unknown,
    MasterToSlave,
    SlaveToMaster,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Byte {
    pub direction: Direction,
    pub timestamp: i32,
    pub duration: Duration,
    pub value: u8,
}

pub struct Bytes<'a, T>
where
    T: Iterator<Item = &'a Sample>,
{
    slots: Slots<'a, T>,
    byte: Option<Byte>,
    bits: usize,
}

impl<'a, T> Iterator for Bytes<'a, T>
where
    T: Iterator<Item = &'a Sample>,
{
    type Item = Byte;

    fn next(&mut self) -> Option<Self::Item> {
        for slot in self.slots.by_ref() {
            self.bits += 1;
            if slot.kind == SlotKind::Reset {
                self.byte = None;
                self.bits = 0;
            } else if let Some(mut byte) = self.byte.take() {
                match (byte.direction, slot.kind) {
                    (_, SlotKind::Reset) => unreachable!(),
                    (Direction::Unknown, SlotKind::One) => {
                        byte.duration = slot.duration
                            + Duration::from_millis(
                                (slot.timestamp - byte.timestamp).try_into().unwrap(),
                            );
                        byte.value = byte.value >> 1 | 0x80;
                        self.byte = Some(byte);
                    }
                    (Direction::Unknown | Direction::MasterToSlave, SlotKind::WriteZero)
                    | (Direction::MasterToSlave, SlotKind::One) => {
                        self.byte = Some(Byte {
                            direction: Direction::MasterToSlave,
                            timestamp: byte.timestamp,
                            duration: slot.duration
                                + Duration::from_millis(
                                    (slot.timestamp - byte.timestamp).try_into().unwrap(),
                                ),
                            value: byte.value >> 1
                                | if matches!(slot.kind, SlotKind::One) {
                                    0x80
                                } else {
                                    0x00
                                },
                        });
                    }
                    (Direction::Unknown | Direction::SlaveToMaster, SlotKind::ReadZero)
                    | (Direction::SlaveToMaster, SlotKind::One) => {
                        self.byte = Some(Byte {
                            direction: Direction::SlaveToMaster,
                            timestamp: byte.timestamp,
                            duration: slot.duration
                                + Duration::from_millis(
                                    (slot.timestamp - byte.timestamp).try_into().unwrap(),
                                ),
                            value: byte.value >> 1
                                | if matches!(slot.kind, SlotKind::One) {
                                    0x80
                                } else {
                                    0x00
                                },
                        });
                    }
                    (Direction::SlaveToMaster, SlotKind::WriteZero) => {
                        self.bits = 1;
                        self.byte = Some(Byte {
                            direction: Direction::MasterToSlave,
                            timestamp: slot.timestamp,
                            duration: slot.duration,
                            value: 0x00,
                        });
                    }
                    (Direction::MasterToSlave, SlotKind::ReadZero) => {
                        self.bits = 1;
                        self.byte = Some(Byte {
                            direction: Direction::SlaveToMaster,
                            timestamp: slot.timestamp,
                            duration: slot.duration,
                            value: 0x00,
                        });
                    }
                }
            } else {
                self.byte = Some(Byte {
                    direction: match slot.kind {
                        SlotKind::Reset => unreachable!(),
                        SlotKind::One => Direction::Unknown,
                        SlotKind::WriteZero => Direction::MasterToSlave,
                        SlotKind::ReadZero => Direction::SlaveToMaster,
                    },
                    timestamp: slot.timestamp,
                    duration: slot.duration,
                    value: if matches!(slot.kind, SlotKind::One) {
                        0x80
                    } else {
                        0x00
                    },
                });
            }

            if self.bits == 8 {
                self.bits = 0;
                return self.byte.take();
            }
        }

        None
    }
}
