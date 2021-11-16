use crate::point::Point;
use std::time::{SystemTime, UNIX_EPOCH};

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RGBColor {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

pub const WHITE_COLOR: RGBColor = RGBColor {
    r: 255,
    g: 255,
    b: 255,
};
pub const BLACK_COLOR: RGBColor = RGBColor { r: 0, g: 0, b: 0 };
pub const RED_COLOR: RGBColor = RGBColor { r: 255, g: 0, b: 0 };
pub const GREEN_COLOR: RGBColor = RGBColor { r: 0, g: 255, b: 0 };
pub const BLUE_COLOR: RGBColor = RGBColor { r: 0, g: 0, b: 255 };

impl RGBColor {
    pub(crate) fn random() -> RGBColor {
        let all = [RED_COLOR, GREEN_COLOR, WHITE_COLOR, BLUE_COLOR];
        all[random_byte() as usize % 4]
    }

    pub(crate) fn intensity(i: f32) -> RGBColor {
        let v = ((i * 255 as f32) as i32 % 255) as u8;
        RGBColor { b: v, g: v, r: v }
    }
}

// Rust doesn't have any built-in random :facepalm:
fn random_byte() -> u8 {
    (SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos()
        % 255) as u8
}

pub struct RGBImage {
    pub pixels: Vec<RGBColor>,
    pub width: u16,
    pub height: u16,
}

impl RGBImage {
    pub fn new(width: u16, height: u16, color: RGBColor) -> Self {
        RGBImage {
            pixels: vec![color; usize::from(width) * usize::from(height)],
            width,
            height,
        }
    }

    pub(crate) fn set_pixel(&mut self, point: Point, color: RGBColor) {
        self.pixels[usize::from(point.x) + usize::from(point.y) * usize::from(self.width)] = color;
    }

    pub fn flip_vertically(&mut self) {
        for y in 0..self.height / 2 {
            for x in 0..self.width {
                let i0 = usize::from(x) + usize::from(y) * usize::from(self.width);
                let i1 =
                    usize::from(x) + usize::from(self.height - 1 - y) * usize::from(self.width);
                self.pixels.swap(i0, i1);
            }
        }
    }
}
