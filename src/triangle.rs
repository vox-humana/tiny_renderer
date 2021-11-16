use crate::point::{barycentric, Point, Vec2};
use crate::rgb_image::{RGBColor, RGBImage, GREEN_COLOR, RED_COLOR};
use std::cmp::{max, min};
use std::mem::swap;

impl RGBImage {
    pub(crate) fn triangle_v(&mut self, points: [Point; 3], color: RGBColor) {
        self.triangle_points(points[0], points[1], points[2], color);
    }

    pub(crate) fn triangle_v2(&mut self, points: [Point; 3], color: RGBColor) {
        let mut b_box_min = Vec2 {
            x: self.width - 1,
            y: self.height - 1,
        };
        let mut b_box_max = Vec2 { x: 0, y: 0 };
        let clamp = b_box_min;
        for i in 0..3 {
            b_box_min.x = max(0, min(b_box_min.x, points[i].x));
            b_box_min.y = max(0, min(b_box_min.y, points[i].y));
            b_box_max.x = min(clamp.x, max(b_box_max.x, points[i].x));
            b_box_max.y = min(clamp.y, max(b_box_max.y, points[i].y));
        }
        for x in b_box_min.x..=b_box_max.x {
            for y in b_box_min.y..=b_box_max.y {
                let pts = points.map(|x| x.as_i32());
                let bc_screen = barycentric(
                    pts,
                    Vec2 {
                        x: x as i32,
                        y: y as i32,
                    },
                );
                if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                    continue;
                }
                self.set_pixel(Point { x, y }, color);
            }
        }
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

        for i in 0..total_height {
            let t0x = sp[0].x as i32;
            let t0y = sp[0].y as i32;
            let t1x = sp[1].x as i32;
            let t1y = sp[1].y as i32;
            let t2x = sp[2].x as i32;
            let t2y = sp[2].y as i32;
            let second_half = i > (t1y - t0y) || (t1y == t0y);

            let segment_height: i32;
            if second_half {
                segment_height = t2y - t1y + 1;
            } else {
                segment_height = t1y - t0y + 1;
            }
            let alpha = i as f32 / total_height as f32;
            let beta: f32;
            if second_half {
                beta = (i - (t1y - t0y)) as f32 / segment_height as f32;
            } else {
                beta = i as f32 / segment_height as f32;
            }

            let ab_calc = |t0x: i32, t0y: i32, t1x: i32, t1y: i32, c: f32| Point {
                x: (t0x + ((t1x - t0x) as f32 * c) as i32) as u16,
                y: (t0y + ((t1y - t0y) as f32 * c) as i32) as u16,
            };
            let mut a = ab_calc(t0x, t0y, t2x, t2y, alpha);

            let mut b: Point;
            if second_half {
                b = ab_calc(t1x, t1y, t2x, t2y, beta);
            } else {
                b = ab_calc(t0x, t0y, t1x, t1y, beta);
            }
            if a.x > b.x {
                swap(&mut a, &mut b);
            }
            for j in a.x..=b.x {
                self.set_pixel(
                    Point {
                        x: j,
                        y: (t0y + i) as u16,
                    },
                    color,
                );
            }
        }
    }

    fn triangle_points(&mut self, p1: Point, p2: Point, p3: Point, color: RGBColor) {
        self.line(p1, p2, color);
        self.line(p2, p3, color);
        self.line(p3, p1, color);
    }
}
