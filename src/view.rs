use crate::graphics::gruvbox::dark as gruvbox;
use crate::{
    Capture, Color, Direction, Draw, Edge, Line, Point, Rect, Sample, Settings, SlotKind, Text,
    Voltage,
};
use core::convert::TryFrom;

const DIVISION_WIDTH: u32 = 100;
const DIVISION_HEIGH: u32 = 80;

pub const WIDTH: u32 = DIVISION_WIDTH * 10;
pub const HEIGHT: u32 = DIVISION_HEIGH * 8;

pub struct View<'a> {
    settings: &'a Settings,
    capture: &'a Capture,
}

impl<'a> View<'a> {
    pub const fn new(settings: &'a Settings, capture: &'a Capture) -> Self {
        Self { settings, capture }
    }

    pub fn sample_to_point(&self, sample: &Sample) -> Option<(u32, u32)> {
        self.timestamp_to_x(sample.timestamp).map_or(None, |x| {
            self.voltage_to_y(sample.voltage)
                .map_or(None, |y| Some((x, y)))
        })
    }

    pub fn timestamp_to_x(&self, timestamp: i32) -> Option<u32> {
        let timestamp_min = self.settings.offset;
        let timestamp_max =
            timestamp_min + i32::try_from(self.settings.period.as_micros()).unwrap() * 10;
        let timestamp_range = timestamp_min..timestamp_max;

        if timestamp_range.contains(&timestamp) {
            let x = (timestamp - self.settings.offset) as u32 * DIVISION_WIDTH
                / u32::try_from(self.settings.period.as_micros()).unwrap();
            Some(x)
        } else {
            None
        }
    }

    pub fn voltage_to_y(&self, voltage: Voltage) -> Option<u32> {
        let voltage_max = self.settings.voltage.as_millivolts() * 4;
        let voltage_min = -voltage_max;
        let voltage_range = voltage_min..voltage_max;

        if voltage_range.contains(&voltage.as_millivolts()) {
            let y = (self.settings.voltage.as_millivolts() * 4 - voltage.as_millivolts()) as u32
                * DIVISION_HEIGH
                / self.settings.voltage.as_millivolts() as u32;
            Some(y)
        } else {
            None
        }
    }

    pub const fn x_to_timestamp(&self, x: u32) -> i32 {
        (x * self.settings.period.as_micros() as u32 / DIVISION_WIDTH) as i32 + self.settings.offset
    }

    #[cfg(disable)]
    pub const fn y_to_voltage(&self, y: u32) -> Voltage {
        let voltage = (y * self.settings.voltage.as_millivolts() as u32 / DIVISION_HEIGH) as i16
            - self.settings.voltage.as_millivolts() * 4;
        Voltage::from_millivolts(-voltage)
    }
}

impl Draw for View<'_> {
    fn draw(&self, frame: &mut [u8], width: u32) {
        Self::draw_axis(frame, width);
        self.draw_bytes(frame, width);
        self.draw_slots(frame, width);
        self.draw_measures(frame, width);
        self.draw_plot(frame, width);
    }
}

impl View<'_> {
    fn draw_bytes(&self, frame: &mut [u8], width: u32) {
        let timestamp_min = self.x_to_timestamp(0);
        let timestamp_max = self.x_to_timestamp(WIDTH - 1);
        let y0 = self
            .voltage_to_y(Voltage::from_millivolts(3500))
            .unwrap_or(0)
            - 20;
        let y1 = self
            .voltage_to_y(Voltage::from_millivolts(-500))
            .unwrap_or(HEIGHT - 1)
            + 20;
        for byte in self.capture.bytes().filter(|byte| {
            byte.timestamp < timestamp_max
                && byte.timestamp + byte.duration.as_millis() as i32 > timestamp_min
        }) {
            let x0 = self.timestamp_to_x(byte.timestamp).unwrap_or(0);
            let x1 = self
                .timestamp_to_x(byte.timestamp + byte.duration.as_millis() as i32)
                .unwrap_or(WIDTH - 1);
            Rect::new()
                .with_x(x0)
                .with_y(y0)
                .with_width(x1 - x0)
                .with_height(y1 - y0)
                .with_color(match byte.direction {
                    Direction::Unknown => gruvbox::BG2.with_alpha(0x44),
                    Direction::SlaveToMaster => gruvbox::ORANGE.with_alpha(0x44),
                    Direction::MasterToSlave => gruvbox::PURPLE.with_alpha(0x44),
                })
                .draw(frame, width);
            let text = format!("{:02x}", byte.value);
            let text = Text::new(&text);
            if text.width() <= x1 - x0 {
                text.with_x(x0 + 2)
                    .with_y(y1 + 2)
                    .with_color(gruvbox::FG4)
                    .draw(frame, width);
            }
        }
    }

    fn draw_slots(&self, frame: &mut [u8], width: u32) {
        let timestamp_min = self.x_to_timestamp(0);
        let timestamp_max = self.x_to_timestamp(WIDTH - 1);
        let y0 = self
            .voltage_to_y(Voltage::from_millivolts(3500))
            .unwrap_or(0);
        let y1 = self
            .voltage_to_y(Voltage::from_millivolts(-500))
            .unwrap_or(HEIGHT - 1);
        for slot in self.capture.slots().filter(|slot| {
            slot.timestamp < timestamp_max
                && slot.timestamp + slot.duration.as_millis() as i32 > timestamp_min
        }) {
            let x0 = self.timestamp_to_x(slot.timestamp).unwrap_or(0);
            let x1 = self
                .timestamp_to_x(slot.timestamp + slot.duration.as_millis() as i32)
                .unwrap_or(WIDTH - 1);
            Rect::new()
                .with_x(x0)
                .with_y(y0)
                .with_width(x1 - x0)
                .with_height(y1 - y0)
                .with_color(gruvbox::BG3.with_alpha(0x80))
                .draw(frame, width);
            let text = Text::new(match slot.kind {
                SlotKind::Reset => "RESET",
                SlotKind::One => "1",
                SlotKind::ReadZero | SlotKind::WriteZero => "0",
            });
            if text.width() <= x1 - x0 {
                text.with_x(x0 + 2)
                    .with_y(y1 + 2)
                    .with_color(match slot.kind {
                        SlotKind::ReadZero => gruvbox::ORANGE,
                        SlotKind::WriteZero => gruvbox::PURPLE,
                        SlotKind::Reset | SlotKind::One => gruvbox::FG4,
                    })
                    .draw(frame, width);
            }
        }
    }

    fn draw_axis(frame: &mut [u8], width: u32) {
        // Draw X axis
        let line = Line::new().with_color(gruvbox::GRAY);
        for x in (0..10).map(|x| x * DIVISION_WIDTH) {
            const Y: u32 = HEIGHT / 2;
            if x > 0 {
                line.clone()
                    .with_x(x)
                    .with_y2(HEIGHT - 1)
                    .draw(frame, width);
            }
            let line = line.clone().with_y1(Y - 2).with_y2(Y + 2);
            for x in (1..5).map(|x| x * DIVISION_WIDTH / 5).map(|o| x + o) {
                line.clone().with_x(x).draw(frame, width);
            }
        }

        // Draw Y axis
        for y in (0..8).map(|y| y * DIVISION_HEIGH) {
            const X: u32 = WIDTH / 2;
            if y > 0 {
                line.clone().with_y(y).with_x2(WIDTH).draw(frame, width);
            }
            let line = line.clone().with_x1(X - 2).with_x2(X + 2);
            for y in (1..4).map(|y| y * DIVISION_WIDTH / 4).map(|o| y + o) {
                line.clone().with_y(y).draw(frame, width);
            }
        }
    }

    fn draw_measures(&self, frame: &mut [u8], width: u32) {
        let mut edges = self
            .capture
            .edges()
            .map(|edge| match edge {
                Edge::Fall { timestamp } | Edge::Raise { timestamp } => timestamp,
            })
            .filter_map(|timestamp| self.timestamp_to_x(timestamp).map(|x| (x, timestamp)));
        if let Some((xa, timestamp_a)) = edges.next() {
            let line = Line::new()
                .with_x(xa)
                .with_y2(HEIGHT - 1)
                .with_color(gruvbox::RED);
            line.draw(frame, width);
            let text = format!("Xa = {} us", timestamp_a);
            Text::new(&text)
                .with_y(HEIGHT - 60)
                .with_color(gruvbox::RED)
                .draw(frame, width);
            if let Some((xb, timestamp_b)) = edges.next() {
                line.with_x(xb).draw(frame, width);
                let text = format!(
                    "Xb = {} us (+{} us)",
                    timestamp_b,
                    timestamp_b - timestamp_a
                );
                Text::new(&text)
                    .with_y(HEIGHT - 40)
                    .with_color(gruvbox::GREEN)
                    .draw(frame, width);
            }
        }
    }

    fn draw_plot(&self, frame: &mut [u8], width: u32) {
        const COLOR_PLOT: Color = gruvbox::YELLOW;

        let mut prev_sample = None;
        let line = Line::new().with_color(COLOR_PLOT);
        for sample in self.capture.iter() {
            if let Some((x, y)) = self.sample_to_point(sample) {
                if let Some(prev_sample) = prev_sample.replace(sample) {
                    if let Some((prev_x, prev_y)) = self.sample_to_point(prev_sample) {
                        line.clone()
                            .with_x1(prev_x)
                            .with_y1(prev_y)
                            .with_x2(x)
                            .with_y2(y)
                            .draw(frame, width);
                    }
                } else {
                    Point::new()
                        .with_x(x)
                        .with_y(y)
                        .with_color(COLOR_PLOT)
                        .draw(frame, width);
                }
            }
        }
    }
}
