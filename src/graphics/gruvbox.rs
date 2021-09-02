#![allow(dead_code)]

use crate::Color;

pub const DARK0_HARD: Color = Color::from_rgb(0x1d, 0x20, 0x21);
pub const DARK0: Color = Color::from_rgb(0x28, 0x28, 0x28);
pub const DARK0_SOFT: Color = Color::from_rgb(0x32, 0x30, 0x2f);
pub const DARK1: Color = Color::from_rgb(0x3c, 0x38, 0x36);
pub const DARK2: Color = Color::from_rgb(0x50, 0x49, 0x45);
pub const DARK3: Color = Color::from_rgb(0x66, 0x5c, 0x54);
pub const DARK4: Color = Color::from_rgb(0x7c, 0x6f, 0x64);
pub const DARK4_256: Color = Color::from_rgb(0x7c, 0x6f, 0x64);

pub const GRAY_245: Color = Color::from_rgb(0x92, 0x83, 0x74);
pub const GRAY_244: Color = Color::from_rgb(0x92, 0x83, 0x74);

pub const LIGHT0_HARD: Color = Color::from_rgb(0xf9, 0xf5, 0xd7);
pub const LIGHT0: Color = Color::from_rgb(0xfb, 0xf1, 0xc7);
pub const LIGHT0_SOFT: Color = Color::from_rgb(0xf2, 0xe5, 0xbc);
pub const LIGHT1: Color = Color::from_rgb(0xeb, 0xdb, 0xb2);
pub const LIGHT2: Color = Color::from_rgb(0xd5, 0xc4, 0xa1);
pub const LIGHT3: Color = Color::from_rgb(0xbd, 0xae, 0x93);
pub const LIGHT4: Color = Color::from_rgb(0xa8, 0x99, 0x84);
pub const LIGHT4_256: Color = Color::from_rgb(0xa8, 0x99, 0x84);

pub const BRIGHT_RED: Color = Color::from_rgb(0xfb, 0x49, 0x34);
pub const BRIGHT_GREEN: Color = Color::from_rgb(0xb8, 0xbb, 0x26);
pub const BRIGHT_YELLOW: Color = Color::from_rgb(0xfa, 0xbd, 0x2f);
pub const BRIGHT_BLUE: Color = Color::from_rgb(0x83, 0xa5, 0x98);
pub const BRIGHT_PURPLE: Color = Color::from_rgb(0xd3, 0x86, 0x9b);
pub const BRIGHT_AQUA: Color = Color::from_rgb(0x8e, 0xc0, 0x7c);
pub const BRIGHT_ORANGE: Color = Color::from_rgb(0xfe, 0x80, 0x19);

pub const NEUTRAL_RED: Color = Color::from_rgb(0xcc, 0x24, 0x1d);
pub const NEUTRAL_GREEN: Color = Color::from_rgb(0x98, 0x97, 0x1a);
pub const NEUTRAL_YELLOW: Color = Color::from_rgb(0xd7, 0x99, 0x21);
pub const NEUTRAL_BLUE: Color = Color::from_rgb(0x45, 0x85, 0x88);
pub const NEUTRAL_PURPLE: Color = Color::from_rgb(0xb1, 0x62, 0x86);
pub const NEUTRAL_AQUA: Color = Color::from_rgb(0x68, 0x9d, 0x6a);
pub const NEUTRAL_ORANGE: Color = Color::from_rgb(0xd6, 0x5d, 0x0e);

pub const FADED_RED: Color = Color::from_rgb(0x9d, 0x00, 0x06);
pub const FADED_GREEN: Color = Color::from_rgb(0x79, 0x74, 0x0e);
pub const FADED_YELLOW: Color = Color::from_rgb(0xb5, 0x76, 0x14);
pub const FADED_BLUE: Color = Color::from_rgb(0x07, 0x66, 0x78);
pub const FADED_PURPLE: Color = Color::from_rgb(0x8f, 0x3f, 0x71);
pub const FADED_AQUA: Color = Color::from_rgb(0x42, 0x7b, 0x58);
pub const FADED_ORANGE: Color = Color::from_rgb(0xaf, 0x3a, 0x03);

pub mod dark {
    use super::Color;

    pub const BG0: Color = super::DARK0;
    pub const BG1: Color = super::DARK1;
    pub const BG2: Color = super::DARK2;
    pub const BG3: Color = super::DARK3;
    pub const BG4: Color = super::DARK4;

    pub const GRAY: Color = super::GRAY_245;

    pub const FG0: Color = super::LIGHT0;
    pub const FG1: Color = super::LIGHT1;
    pub const FG2: Color = super::LIGHT2;
    pub const FG3: Color = super::LIGHT3;
    pub const FG4: Color = super::LIGHT4;

    pub const FG4_256: Color = super::LIGHT4_256;

    pub const RED: Color = super::BRIGHT_RED;
    pub const GREEN: Color = super::BRIGHT_GREEN;
    pub const YELLOW: Color = super::BRIGHT_YELLOW;
    pub const BLUE: Color = super::BRIGHT_BLUE;
    pub const PURPLE: Color = super::BRIGHT_PURPLE;
    pub const AQUA: Color = super::BRIGHT_AQUA;
    pub const ORANGE: Color = super::BRIGHT_ORANGE;
}

pub mod light {
    use super::Color;

    pub const BG0: Color = super::LIGHT0;
    pub const BG1: Color = super::LIGHT1;
    pub const BG2: Color = super::LIGHT2;
    pub const BG3: Color = super::LIGHT3;
    pub const BG4: Color = super::LIGHT4;

    pub const GRAY: Color = super::GRAY_244;

    pub const FG0: Color = super::DARK0;
    pub const FG1: Color = super::DARK1;
    pub const FG2: Color = super::DARK2;
    pub const FG3: Color = super::DARK3;
    pub const FG4: Color = super::DARK4;

    pub const FG4_256: Color = super::DARK4_256;

    pub const RED: Color = super::FADED_RED;
    pub const GREEN: Color = super::FADED_GREEN;
    pub const YELLOW: Color = super::FADED_YELLOW;
    pub const BLUE: Color = super::FADED_BLUE;
    pub const PURPLE: Color = super::FADED_PURPLE;
    pub const AQUA: Color = super::FADED_AQUA;
    pub const ORANGE: Color = super::FADED_ORANGE;
}
