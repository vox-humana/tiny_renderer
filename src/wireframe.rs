use crate::point::{Point, Vec2, Vec3};
use crate::rgb_image::{RGBColor, RGBImage};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub(crate) type Vertex3 = Vec3<f32>;

pub struct WireframeModel {
    vertexes: Vec<Vertex3>,
    faces: Vec<[usize; 3]>,
}

impl FromStr for Vertex3 {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let mut parse_float = || {
            it.next()
                .expect("point value")
                .parse::<f32>()
                .expect("float value")
        };
        let x = parse_float();
        let y = parse_float();
        let z = parse_float();
        Ok(Vertex3 { x, y, z })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex3_from_str() {
        let s = "v 0.123 0.234 0.345 1.0";
        let v = Vertex3::from_str(&s[2..]).unwrap();
        assert_eq!(v.x, 0.123);
        assert_eq!(v.y, 0.234);
        assert_eq!(v.z, 0.345);
    }

    #[test]
    fn test_face_from_str() {
        let s = "f 1193/1240/1193 1180/1227/1180 1179/1226/1179";
        let face = WireframeModel::face_from_str(&s[2..]);
        assert_eq!(face, [1192, 1179, 1178]);
    }
}

impl WireframeModel {
    fn face_from_str(s: &str) -> [usize; 3] {
        // we use only 1 index
        fn first_vertex_index(s: &str) -> usize {
            let v = s
                .split("/")
                .next()
                .expect("index value")
                .parse::<i32>()
                .expect("usize value")
                - 1; // in wavefront obj all indices start at 1, not zero
            assert!(v >= 0, "only positive values");
            return v as usize;
        }
        let mut it = s.split_ascii_whitespace();
        let mut parse_index = || first_vertex_index(it.next().expect("coordinates"));
        return [parse_index(), parse_index(), parse_index()];
    }

    pub fn from_file(path: String) -> WireframeModel {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut vertexes: Vec<Vertex3> = vec![];
        let mut faces: Vec<[usize; 3]> = vec![];
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let s = line.trim();
            if s.is_empty() {
                continue;
            }
            if s.starts_with("v ") {
                let v = Vertex3::from_str(&s[2..]).unwrap();
                vertexes.push(v);
            } else if s.starts_with("f ") {
                let f = Self::face_from_str(&s[2..]);
                faces.push(f);
            } else {
                println!("unsupported line {}", s)
            }
        }

        WireframeModel { vertexes, faces }
    }
}

impl RGBImage {
    pub fn render_frame(&mut self, wireframe: WireframeModel, color: RGBColor) {
        for face in wireframe.faces {
            for j in 0..3 {
                fn normalize(value: f32, side: u16) -> u16 {
                    ((value + 1.0) * (side as f32 - 1.0) / 2.0) as u16
                }
                let v0 = wireframe.vertexes[face[j]];
                let v1 = wireframe.vertexes[face[(j + 1) % 3]];
                let x0 = normalize(v0.x, self.width);
                let y0 = normalize(v0.y, self.height);
                let x1 = normalize(v1.x, self.width);
                let y1 = normalize(v1.y, self.height);
                self.line(Point { x: x0, y: y0 }, Point { x: x1, y: y1 }, color);
            }
        }
    }

    pub fn render(&mut self, wireframe: WireframeModel) {
        for face in wireframe.faces {
            let projection = |j: usize| {
                let world_coords: Vec3<f32> = wireframe.vertexes[face[j]];
                Vec2::<u16> {
                    x: ((world_coords.x + 1.0) * self.width as f32 / 2.0) as u16,
                    y: ((world_coords.y + 1.0) * self.height as f32 / 2.0) as u16,
                }
            };
            let pts = (0..3).map(projection).collect();
            self.triangle_v2(pts, RGBColor::random());
        }
    }
}

impl FromIterator<Point> for [Point; 3] {
    fn from_iter<T: IntoIterator<Item = Vec2<u16>>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
    }
}
