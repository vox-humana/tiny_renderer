use crate::point::Point;
use crate::rgb_image::{RGBColor, RGBImage, GREEN_COLOR, RED_COLOR};
use std::mem::swap;

impl RGBImage {
    pub(crate) fn triangle_v(&mut self, points: [Point; 3], color: RGBColor) {
        self.triangle(points[0], points[1], points[2], color);
    }

    pub(crate) fn triangle_v_sorted(&mut self, points: [Point; 3]) {
        let mut sp = points;
        sp.sort_by(|p1, p2| p1.y.cmp(&p2.y));
        self.line(sp[0], sp[1], GREEN_COLOR);
        self.line(sp[1], sp[2], GREEN_COLOR);
        self.line(sp[2], sp[0], RED_COLOR);
    }

    pub(crate) fn triangle_filed(&mut self, points: [Point; 3], color: RGBColor) {
        let mut sp = points;
        sp.sort_by(|p1, p2| p1.y.cmp(&p2.y));

        let total_height = sp[2].y as i32 - sp[0].y as i32;
        for y in sp[0].y..=sp[1].y {
            let segment_height = sp[1].y as i32 - sp[0].y as i32 + 1;
            let alpha = (y as i32 - sp[0].y as i32) as f32 / total_height as f32;
            let beta = (y as i32 - sp[0].y as i32) as f32 / segment_height as f32;
            let mut A = sp[0].shift(
                ((sp[2].x as i32 - sp[0].x as i32) as f32 * alpha) as i32,
                ((sp[2].y as i32 - sp[0].y as i32) as f32 * alpha) as i32,
            );
            let mut B = sp[0].shift(
                ((sp[1].x as i32 - sp[0].x as i32) as f32 * beta) as i32,
                ((sp[1].y as i32 - sp[0].y as i32) as f32 * beta) as i32,
            );
            if A.x > B.x {
                swap(&mut A, &mut B);
            }
            // self.set_pixel(Point{x: A.x, y}, RED_COLOR);
            // self.set_pixel(Point{x: B.x, y}, GREEN_COLOR);
            for j in A.x..=B.x {
                self.set_pixel(Point { x: j, y }, color);
            }
        }
        // TODO: second part
    }

    fn triangle(&mut self, p1: Point, p2: Point, p3: Point, color: RGBColor) {
        self.line(p1, p2, color);
        self.line(p2, p3, color);
        self.line(p3, p1, color);
    }
}
