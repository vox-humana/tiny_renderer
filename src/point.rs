use std::ops::{Mul, Sub};

#[derive(Clone, Copy)]
pub(crate) struct Vec2<T> {
    pub(crate) x: T,
    pub(crate) y: T,
}

pub(crate) type Point = Vec2<u16>;

#[derive(Clone, Copy)]
pub(crate) struct Vec3<T> {
    pub(crate) x: T,
    pub(crate) y: T,
    pub(crate) z: T,
}

impl Vec2<u16> {
    pub(crate) fn from(x: i32, y: i32) -> Vec2<u16> {
        assert!(x >= 0);
        assert!(y >= 0);
        Point {
            x: x as u16,
            y: y as u16,
        }
    }
    pub(crate) fn shift(self, dx: i32, dy: i32) -> Vec2<u16> {
        Point::from(self.x as i32 + dx, self.y as i32 + dy)
    }

    pub(crate) fn as_i32(self) -> Vec2<i32> {
        Vec2 {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

impl Vec3<u16> {
    pub(crate) fn as_i32(self) -> Vec3<i32> {
        Vec3 {
            x: self.x as i32,
            y: self.y as i32,
            z: self.z as i32,
        }
    }
}

impl Vec3<f32> {
    fn norm(self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub(crate) fn normalize(&mut self) {
        let n = self.norm();
        self.x /= n;
        self.y /= n;
        self.z /= n;
    }

    pub(crate) fn as_u16(self) -> Vec3<u16> {
        Vec3 {
            x: self.x as u16,
            y: self.y as u16,
            z: self.z as u16,
        }
    }
}

pub(crate) fn barycentric(pts: [Vec2<i32>; 3], p: Vec2<i32>) -> Vec3<f32> {
    let u = cross(
        Vec3 {
            x: pts[2].x - pts[0].x,
            y: pts[1].x - pts[0].x,
            z: pts[0].x - p.x,
        },
        Vec3 {
            x: pts[2].y - pts[0].y,
            y: pts[1].y - pts[0].y,
            z: pts[0].y - p.y,
        },
    );
    if u.z.abs() < 1 {
        return Vec3 {
            x: -1.0,
            y: 1.0,
            z: 1.0,
        };
    }
    return Vec3 {
        x: 1.0 - (u.x + u.y) as f32 / u.z as f32,
        y: u.y as f32 / u.z as f32,
        z: u.x as f32 / u.z as f32,
    };
}

pub(crate) fn barycentric_3d(pts: [Vec3<i32>; 3], p: Vec2<i32>) -> Vec3<f32> {
    let pts_2d = pts.map(|p| Vec2 { x: p.x, y: p.y });
    barycentric(pts_2d, p)
}

pub(crate) fn cross<T: Mul<Output = T> + Sub<Output = T> + Copy>(
    v1: Vec3<T>,
    v2: Vec3<T>,
) -> Vec3<T> {
    Vec3 {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    }
}

pub(crate) fn diff<T: Sub<Output = T>>(v1: Vec3<T>, v2: Vec3<T>) -> Vec3<T> {
    Vec3 {
        x: v1.x - v2.x,
        y: v1.y - v2.y,
        z: v1.z - v2.z,
    }
}
