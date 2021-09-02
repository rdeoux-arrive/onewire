use crate::{Color, Draw, Line};

#[derive(Clone, Debug, PartialEq)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: Color,
}

impl Rect {
    pub const fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            color: Color::BLACK,
        }
    }

    pub const fn with_x(self, x: u32) -> Self {
        Self { x, ..self }
    }

    pub const fn with_y(self, y: u32) -> Self {
        Self { y, ..self }
    }

    pub const fn with_width(self, width: u32) -> Self {
        Self { width, ..self }
    }

    pub const fn with_height(self, height: u32) -> Self {
        Self { height, ..self }
    }

    pub const fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }
}

impl Draw for Rect {
    fn draw(&self, frame: &mut [u8], width: u32) {
        let mut line = Line::new()
            .with_x1(self.x)
            .with_x2(self.x + self.width)
            .with_color(self.color);
        for y in 0..=self.height {
            let y = self.y + y;
            line = line.with_y1(y).with_y2(y);
            line.draw(frame, width)
        }
    }
}
