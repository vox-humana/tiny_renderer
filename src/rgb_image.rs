#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct RGBColor {
    pub b: u8,
    pub g: u8,
    pub r: u8,
}

pub const WHITE_COLOR: RGBColor = RGBColor { r: 255, g: 255, b: 255 };
pub const BLACK_COLOR: RGBColor = RGBColor { r: 0, g: 0, b: 0 };
pub const RED_COLOR: RGBColor = RGBColor { r: 255, g: 0, b: 0 };
pub const GREEN_COLOR: RGBColor = RGBColor { r: 0, g: 255, b: 0 };

pub struct RGBImage {
    pub pixels: Vec<RGBColor>,
    pub width: u16,
    pub height: u16,
}

pub(crate) struct Point {
    pub(crate) x: u16,
    pub(crate) y: u16,
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

    pub fn flip(&mut self) {
        for y in 0 .. self.height / 2 {
            for x in 0 .. self.width {
                let i0 = usize::from(x) + usize::from(y) * usize::from(self.width);
                let i1 = usize::from(x) + usize::from(self.height - 1 - y) * usize::from(self.width);
                self.pixels.swap(i0, i1);
            }
        }
    }
}
