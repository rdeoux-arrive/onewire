use crate::{Color, Draw};
use core::convert::TryFrom;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl Point {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            color: Color::BLACK,
        }
    }

    #[must_use]
    pub const fn with_x(self, x: u32) -> Self {
        Self { x, ..self }
    }

    #[must_use]
    pub const fn with_y(self, y: u32) -> Self {
        Self { y, ..self }
    }

    #[must_use]
    pub const fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }
}

impl Draw for Point {
    fn draw(&self, frame: &mut [u8], width: u32) {
        if self.x < width && self.color.alpha != 0 {
            let x = usize::try_from(self.x).unwrap_or(usize::MAX);
            let y = usize::try_from(self.y).unwrap_or(usize::MAX);
            let width = usize::try_from(width).unwrap();
            if let Some(chunk) = frame.chunks_exact_mut(4).nth(x + y * width) {
                if self.color.alpha == 0xff {
                    chunk.copy_from_slice(&self.color.to_bytes());
                } else {
                    let q = u16::from(self.color.alpha);
                    let nq = 0x100 - q;
                    chunk[0] =
                        (u16::from(self.color.red) * q + u16::from(chunk[0]) * nq).to_be_bytes()[0];
                    chunk[1] = (u16::from(self.color.green) * q + u16::from(chunk[1]) * nq)
                        .to_be_bytes()[0];
                    chunk[2] = (u16::from(self.color.blue) * q + u16::from(chunk[2]) * nq)
                        .to_be_bytes()[0];
                }
            }
        }
    }
}

#[test]
fn draw_alpha() {
    const RED: Color = Color::from_rgb(0xff, 0x00, 0x00);
    const GREEN: Color = Color::from_rgb(0x00, 0xff, 0x00);
    const YELLOW: Color = Color::from_rgb(0x7f, 0x7f, 0x00);

    let mut frame = RED.to_bytes();
    let point = Point::new().with_color(GREEN.with_alpha(0x80));
    point.draw(&mut frame, 1);
    assert_eq!(frame, YELLOW.to_bytes());
}
