use crate::point::Point;
use crate::rgb_image::RGBColor;
use crate::rgb_image::RGBImage;
use std::mem::swap;

// Lesson 1: Bresenham’s Line Drawing Algorithm
// https://github.com/ssloy/tinyrenderer/wiki/Lesson-1:-Bresenham’s-Line-Drawing-Algorithm

impl RGBImage {
    /*
    pub(crate) fn line1(&mut self, start: Point, end: Point, color: RGBColor) {
        for i in 0..100 {
            let point = Point {
                x: start.x + (end.x - start.x) * i / 100,
                y: start.y + (end.y - start.y) * i / 100
            };
            self.set_pixel(point, color);
        }
    }
    */

    pub(crate) fn line(&mut self, start: Point, end: Point, color: RGBColor) {
        let mut x0: i32 = start.x as i32;
        let mut y0: i32 = start.y as i32;
        let mut x1: i32 = end.x as i32;
        let mut y1: i32 = end.y as i32;
        let steep;
        if (x1 - x0).abs() < (y1 - y0).abs() {
            swap(&mut x0, &mut y0);
            swap(&mut x1, &mut y1);
            steep = true;
        } else {
            steep = false;
        }

        if x0 > x1 {
            swap(&mut x0, &mut x1);
            swap(&mut y0, &mut y1);
        }

        let dx = x1 - x0;
        let dy = y1 - y0;
        let derror2 = dy.abs() * 2;
        let mut error2 = 0;
        let mut y = y0;
        for x in x0..=x1 {
            if steep {
                self.set_pixel(Point::from(y, x), color);
            } else {
                self.set_pixel(Point::from(x, y), color);
            }
            error2 += derror2;
            if error2 > dx {
                if y1 > y0 {
                    y += 1;
                } else {
                    y -= 1;
                }
                error2 -= dx * 2;
            }
        }
    }
}
