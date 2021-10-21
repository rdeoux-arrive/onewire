use crate::{Color, Draw, Point};
use core::convert::TryFrom;
use image::{load_from_memory, GenericImageView};

const TEXT_PNG: &[u8] = include_bytes!("text.png");
const TEXT_CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";

#[derive(Clone, Debug, PartialEq)]
pub struct Text<'a> {
    pub x: u32,
    pub y: u32,
    pub text: &'a str,
    pub color: Color,
}

impl<'a> Text<'a> {
    pub const fn new(text: &'a str) -> Self {
        Self {
            x: 0,
            y: 0,
            text,
            color: Color::BLACK,
        }
    }

    pub const fn with_x(self, x: u32) -> Self {
        Self { x, ..self }
    }

    pub const fn with_y(self, y: u32) -> Self {
        Self { y, ..self }
    }

    pub const fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn width(&self) -> u32 {
        load_from_memory(TEXT_PNG)
            .map(|img| {
                let w = img.width() / u32::try_from(TEXT_CHARS.len()).unwrap_or(u32::MAX);
                debug_assert!(w > 0);
                w * u32::try_from(self.text.len()).unwrap_or(u32::MAX)
            })
            .unwrap_or(0)
    }
}

impl Draw for Text<'_> {
    fn draw(&self, frame: &mut [u8], width: u32) {
        if let (Ok(img), Ok(text_len)) =
            (load_from_memory(TEXT_PNG), u32::try_from(TEXT_CHARS.len()))
        {
            let img = img.to_luma8();
            let char_w = img.width() / text_len;
            let char_h = img.height();

            for (pos, c) in self
                .text
                .chars()
                .enumerate()
                .filter_map(|(pos, c)| u32::try_from(pos).ok().map(|pos| (pos, c)))
            {
                let off_x = self.x + pos * char_w;
                if let Some(Ok(pos)) = TEXT_CHARS.find(c).map(u32::try_from) {
                    for x in 0..char_w {
                        for y in 0..char_h {
                            if img.get_pixel(pos * char_w + x, y).0[0] == 0 {
                                Point::new()
                                    .with_x(off_x + x)
                                    .with_y(self.y + y)
                                    .with_color(self.color)
                                    .draw(frame, width);
                            }
                        }
                    }
                }
            }
        }
    }
}
