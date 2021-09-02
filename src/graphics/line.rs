use crate::{Color, Draw, Point};

#[derive(Clone, Debug, PartialEq)]
pub struct Line {
    pub x1: u32,
    pub y1: u32,
    pub x2: u32,
    pub y2: u32,
    pub color: Color,
}

impl Line {
    pub const fn new() -> Self {
        Self {
            x1: 0,
            y1: 0,
            x2: 0,
            y2: 0,
            color: Color::BLACK,
        }
    }

    pub const fn with_x(self, x: u32) -> Self {
        self.with_x1(x).with_x2(x)
    }

    pub const fn with_y(self, y: u32) -> Self {
        self.with_y1(y).with_y2(y)
    }

    pub const fn with_x1(self, x1: u32) -> Self {
        Self { x1, ..self }
    }

    pub const fn with_y1(self, y1: u32) -> Self {
        Self { y1, ..self }
    }

    pub const fn with_x2(self, x2: u32) -> Self {
        Self { x2, ..self }
    }

    pub const fn with_y2(self, y2: u32) -> Self {
        Self { y2, ..self }
    }

    pub const fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }
}

impl Draw for Line {
    fn draw(&self, buf: &mut [u8], width: u32) {
        if self.x1 == self.x2 {
            let (y1, y2) = if self.y1 < self.y2 {
                (self.y1, self.y2)
            } else {
                (self.y2, self.y1)
            };
            let point = Point::new().with_x(self.x1).with_color(self.color);
            for y in y1..=y2 {
                point.clone().with_y(y).draw(buf, width)
            }
        } else if self.y1 == self.y2 {
            let (x1, x2) = if self.x1 < self.x2 {
                (self.x1, self.x2)
            } else {
                (self.x2, self.x1)
            };
            let point = Point::new().with_y(self.y1).with_color(self.color);
            for x in x1..=x2 {
                point.clone().with_x(x).draw(buf, width)
            }
        } else {
            const fn distance(a: u32, b: u32) -> u32 {
                if a < b {
                    b - a
                } else {
                    a - b
                }
            }
            let dx = distance(self.x1, self.x2);
            let dy = distance(self.y1, self.y2);
            if dx > 1 || dy > 1 {
                let x3 = (self.x1 + self.x2) / 2;
                let y3 = (self.y1 + self.y2) / 2;
                self.clone().with_x2(x3).with_y2(y3).draw(buf, width);
                self.clone().with_x1(x3).with_y1(y3).draw(buf, width);
            } else {
                let point = Point::new().with_color(self.color);
                point
                    .clone()
                    .with_x(self.x1)
                    .with_y(self.y1)
                    .draw(buf, width);
                point.with_x(self.x2).with_y(self.y2).draw(buf, width);
            }
        }
    }
}
