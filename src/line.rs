use crate::rgb_image::{Point, RGBColor};
use crate::rgb_image::RGBImage;

impl RGBImage {
    pub(crate) fn line(&mut self, start: Point, end: Point, color: RGBColor) {
        for i in 0..100 {
            let point = Point {
                x: start.x + (end.x - start.x) * i / 100,
                y: start.y + (end.y - start.y) * i / 100
            };
            self.set_pixel(point, color);
        }
    }
}