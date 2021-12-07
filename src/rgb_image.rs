use crate::point::Point;

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
        RGBColor {
            b: random_byte(),
            g: random_byte(),
            r: random_byte(),
        }
    }

    pub(crate) fn intensity(i: f32) -> RGBColor {
        let v = (i * 255.0) as u8;
        RGBColor { b: v, g: v, r: v }
    }

    pub(crate) fn with_intensity(self, i: f32) -> RGBColor {
        RGBColor {
            b: (self.b as f32 * i) as u8,
            g: (self.g as f32 * i) as u8,
            r: (self.r as f32 * i) as u8,
        }
    }
}

// Rust standard library doesn't have any built-in pseudo random generator :facepalm:
// Let's use some simple one
// https://en.wikipedia.org/wiki/Lehmer_random_number_generator
fn lcg_parkmiller(state: u32) -> u32 {
    ((state as u64) * 48271 % 0x7fffffff) as u32
}

fn random_byte() -> u8 {
    static mut STATE: u32 = 13;
    unsafe {
        STATE = lcg_parkmiller(STATE);
        return (STATE % 255) as u8;
    }
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
