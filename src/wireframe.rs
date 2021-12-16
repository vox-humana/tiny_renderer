use crate::point::Vec3;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub(crate) type Vertex3 = Vec3<f32>;

#[derive(PartialEq, Debug, Copy, Clone)]
pub(crate) struct Face {
    pub(crate) vertex_index: usize,
    pub(crate) texture_index: usize,
    pub(crate) norm_index: usize,
}

pub(crate) struct WireframeModel {
    pub(crate) vertexes: Vec<Vertex3>,
    pub(crate) faces: Vec<[Face; 3]>,
    pub(crate) texture_coord: Vec<(f32, f32)>,
    pub(crate) norm: Vec<Vec3<f32>>,
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
        let s = "f 1193/1240/1193 1180/1227/1180 1179/1226/1178";
        let face = WireframeModel::face_from_str(&s[2..]);
        let expected: [Face; 3] = [
            Face {
                vertex_index: 1192,
                texture_index: 1239,
                norm_index: 1192,
            },
            Face {
                vertex_index: 1179,
                texture_index: 1226,
                norm_index: 1179,
            },
            Face {
                vertex_index: 1178,
                texture_index: 1225,
                norm_index: 1177,
            },
        ];
        assert_eq!(face, expected);
    }

    #[test]
    fn test_texture_coord_from_str() {
        let s = "vt  0.532 0.923 0.000";
        let face = WireframeModel::texture_coord_from_str(&s[2..]);
        assert_eq!(face, (0.532, 0.923));
    }

    #[test]
    fn test_texture_norm_from_str() {
        let s = "vn  -0.319 -0.065 0.946";
        let norm = WireframeModel::norm_from_str(&s[2..]);
        assert_eq!(
            norm,
            Vec3 {
                x: -0.319,
                y: -0.065,
                z: 0.946
            }
        );
    }
}

impl WireframeModel {
    fn texture_coord_from_str(s: &str) -> (f32, f32) {
        let mut it = s.split_ascii_whitespace();
        let x = it
            .next()
            .expect("x value")
            .parse::<f32>()
            .expect("float value");
        let y = it
            .next()
            .expect("y value")
            .parse::<f32>()
            .expect("float value");
        return (x, y);
    }

    fn face_from_str(s: &str) -> [Face; 3] {
        fn index(s: &str) -> usize {
            let i = s.parse::<i32>().expect("usize value") - 1; // in wavefront obj all indices start at 1, not zero
            return i as usize;
        }

        fn face(s: &str) -> Face {
            let mut it = s.split("/");
            return Face {
                vertex_index: index(it.next().expect("uv index value")),
                texture_index: index(it.next().expect("vertex index value")),
                norm_index: index(it.next().expect("normal index value")),
            };
        }
        let mut it = s.split_ascii_whitespace();
        let mut parse_face = || face(it.next().expect("coordinates"));
        return [parse_face(), parse_face(), parse_face()];
    }

    fn norm_from_str(s: &str) -> Vec3<f32> {
        let mut it = s.split_ascii_whitespace();
        let x = it
            .next()
            .expect("x value")
            .parse::<f32>()
            .expect("float value");
        let y = it
            .next()
            .expect("y value")
            .parse::<f32>()
            .expect("float value");
        let z = it
            .next()
            .expect("z value")
            .parse::<f32>()
            .expect("float value");
        Vec3 { x, y, z }
    }

    pub fn from_file(path: String) -> WireframeModel {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut vertexes: Vec<Vertex3> = vec![];
        let mut faces: Vec<[Face; 3]> = vec![];
        let mut texture_coord: Vec<(f32, f32)> = vec![];
        let mut norm: Vec<Vec3<f32>> = vec![];

        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let s = line.trim();
            if s.is_empty() || s.starts_with("#") {
                continue;
            }
            if s.starts_with("v ") {
                let v = Vertex3::from_str(&s[2..]).unwrap();
                vertexes.push(v);
            } else if s.starts_with("f ") {
                let f = Self::face_from_str(&s[2..]);
                faces.push(f);
            } else if s.starts_with("vt") {
                let c = Self::texture_coord_from_str(&s[2..]);
                texture_coord.push(c);
            } else if s.starts_with("vn") {
                let v = Self::norm_from_str(&s[2..]);
                norm.push(v);
            } else {
                println!("unsupported line {}", s)
            }
        }

        WireframeModel {
            vertexes,
            faces,
            texture_coord,
            norm,
        }
    }
}
