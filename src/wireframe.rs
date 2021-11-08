use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use crate::rgb_image::{Point, RGBColor, RGBImage};

#[derive(Copy, Clone)]
struct Vertex3 {
    x: f32,
    y: f32,
    z: f32
}

pub struct WireframeModel {
    vertexes: Vec<Vertex3>,
    faces: Vec<[usize; 3]>
}

impl FromStr for Vertex3 {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_ascii_whitespace();
        let xs = it.next().unwrap();
        let x = f32::from_str(xs)?;
        let ys = it.next().unwrap();
        let y = f32::from_str(ys)?;
        let zs = it.next().unwrap();
        let z = f32::from_str(zs)?;
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
        // Seems like we need only first item
        // https://github.com/ssloy/tinyrenderer/blob/f6fecb7ad493264ecd15e230411bfb1cca539a12/model.cpp#L26
        let face = WireframeModel::face_from_str(&s[2..]);
        assert_eq!(face, [1192, 1179, 1178]);
    }
}

impl WireframeModel {
    fn face_from_str(s: &str) -> [usize; 3] {
        fn vertex_indexes(s: &str) -> [usize; 3] {
            let mut it = s.split("/").filter(|&x| !x.is_empty());
            let s0 = it.next().unwrap();
            let v0 = usize::from_str(s0).unwrap();
            let s1 = it.next().unwrap();
            let v1 = usize::from_str(s1).unwrap();
            let s2 = it.next().unwrap();
            let v2 = usize::from_str(s2).unwrap();
            return [v0 - 1, v1 - 1, v2 - 1];
        }
        let mut it = s.split_ascii_whitespace();
        let v0 = vertex_indexes(it.next().unwrap())[0];
        let v1 = vertex_indexes(it.next().unwrap())[0];
        let v2 = vertex_indexes(it.next().unwrap())[0];
        return [v0, v1, v2];
    }

    pub fn from_file(path: String) -> WireframeModel {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut vertexes: Vec<Vertex3> = vec![];
        let mut faces: Vec<[usize; 3]> = vec![];
        for (_, line) in reader.lines().enumerate() {
            let s = line.unwrap();
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
    pub fn render(&mut self, wireframe: WireframeModel, color: RGBColor) {
        for face in wireframe.faces {
            for j in 0 .. 3 {
                let v0 = wireframe.vertexes[face[j]];
                let v1 = wireframe.vertexes[face[(j + 1) % 3]];
                let x0 = (v0.x + 1.0) * f32::from(self.width) / 2.0;
                let y0 = (v0.y + 1.0) * f32::from(self.height) / 2.0;
                let x1 = (v1.x + 1.0) * f32::from(self.width) / 2.0;
                let y1 = (v1.y + 1.0) * f32::from(self.height) / 2.0;
                let mut start = Point { x: x0 as u16, y: y0 as u16};
                start.normalize(self.width - 1 , self.height - 1);
                let mut end = Point { x: x1 as u16, y: y1 as u16};
                end.normalize(self.width - 1 , self.height - 1);
                self.line(start, end, color);
            }
        }
    }
}

impl Point {
    fn normalize(&mut self, max_x: u16, max_y: u16) {
        self.x = min(self.x, max_x);
        self.y = min(self.y, max_y);
    }
}