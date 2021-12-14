use crate::matrix::{Matrix, ViewPort};
use crate::point::{cross, diff, Point, Vec2, Vec3};
use crate::rgb_image::{RGBColor, RGBImage};
use image::{DynamicImage, GenericImageView};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub(crate) type Vertex3 = Vec3<f32>;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Face {
    vertex_index: usize,
    texture_index: usize,
}

pub struct WireframeModel {
    vertexes: Vec<Vertex3>,
    faces: Vec<[Face; 3]>,
    texture_coord: Vec<(f32, f32)>,
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
        let expected: [Face; 3] = [
            Face {
                vertex_index: 1192,
                texture_index: 1239,
            },
            Face {
                vertex_index: 1179,
                texture_index: 1226,
            },
            Face {
                vertex_index: 1178,
                texture_index: 1225,
            },
        ];
        assert_eq!(face, expected);
    }

    #[test]
    fn test_texture_coord_from_str() {
        let s = "vt  0.532 0.923 0.000";
        let face = WireframeModel::texture_coord(&s[2..]);
        assert_eq!(face, (0.532, 0.923));
    }
}

impl WireframeModel {
    fn texture_coord(s: &str) -> (f32, f32) {
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
        // we use only first 2 indexes
        fn index(s: &str) -> usize {
            let i = s.parse::<i32>().expect("usize value") - 1; // in wavefront obj all indices start at 1, not zero
            return i as usize;
        }

        fn face(s: &str) -> Face {
            let mut it = s.split("/");
            return Face {
                vertex_index: index(it.next().expect("uv index value")),
                texture_index: index(it.next().expect("vertex index value")),
            };
        }
        let mut it = s.split_ascii_whitespace();
        let mut parse_face = || face(it.next().expect("coordinates"));
        return [parse_face(), parse_face(), parse_face()];
    }

    pub fn from_file(path: String) -> WireframeModel {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);

        let mut vertexes: Vec<Vertex3> = vec![];
        let mut faces: Vec<[Face; 3]> = vec![];
        let mut texture_coord: Vec<(f32, f32)> = vec![];
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
            } else if s.starts_with("vt") {
                let c = Self::texture_coord(&s[2..]);
                texture_coord.push(c);
            } else {
                println!("unsupported line {}", s)
            }
        }

        WireframeModel {
            vertexes,
            faces,
            texture_coord,
        }
    }
}

impl RGBImage {
    pub fn render_frame(&mut self, wireframe: WireframeModel, color: RGBColor) {
        for face in wireframe.faces {
            for j in 0..3 {
                fn normalize(value: f32, side: u16) -> u16 {
                    ((value + 1.0) * (side as f32 - 1.0) / 2.0) as u16
                }
                let v0 = wireframe.vertexes[face[j].vertex_index];
                let v1 = wireframe.vertexes[face[(j + 1) % 3].vertex_index];
                let x0 = normalize(v0.x, self.width);
                let y0 = normalize(v0.y, self.height);
                let x1 = normalize(v1.x, self.width);
                let y1 = normalize(v1.y, self.height);
                self.line(Point { x: x0, y: y0 }, Point { x: x1, y: y1 }, color);
            }
        }
    }

    pub fn render_random(&mut self, wireframe: WireframeModel) {
        for face in wireframe.faces {
            let world_coords = face.map(|f| wireframe.vertexes[f.vertex_index]);
            let pts = RGBImage::screen_triangle(world_coords, self.width, self.height);
            self.triangle_filed(pts, RGBColor::random());
        }
    }

    pub(crate) fn render_light(&mut self, wireframe: WireframeModel, light_dir: Vec3<f32>) {
        for face in wireframe.faces {
            let world_coords = face.map(|f| wireframe.vertexes[f.vertex_index]);
            let mut n = cross(
                diff(world_coords[2], world_coords[0]),
                diff(world_coords[1], world_coords[0]),
            );
            n.normalize();
            let intensity = light_dir.x * n.x + light_dir.y * n.y + light_dir.z * n.z;

            if intensity > 0.0 {
                self.triangle_filed(
                    RGBImage::screen_triangle(world_coords, self.width, self.height),
                    RGBColor::intensity(intensity),
                );
            }
        }
    }

    pub(crate) fn render_z_buffer(&mut self, wireframe: WireframeModel, light_dir: Vec3<f32>) {
        let z_buffer_size: i32 = self.width as i32 * self.height as i32;
        let mut z_buffer: Vec<f32> = (0..z_buffer_size).map(|_x| -1.0).collect();

        for face in wireframe.faces {
            let world_coords = face.map(|f| wireframe.vertexes[f.vertex_index]);
            let mut n = cross(
                diff(world_coords[2], world_coords[0]),
                diff(world_coords[1], world_coords[0]),
            );
            n.normalize();
            let intensity = light_dir.x * n.x + light_dir.y * n.y + light_dir.z * n.z;

            if intensity > 0.0 {
                let pts = RGBImage::screen_triangle_3d(world_coords, self.width, self.height);
                self.triangle_z_buffer(pts, &mut z_buffer, RGBColor::intensity(intensity));
            }
        }
    }

    pub(crate) fn render_z_buffer_texture(
        &mut self,
        wireframe: WireframeModel,
        texture: DynamicImage,
        light_dir: Vec3<f32>,
    ) {
        let w = self.width;
        let h = self.height;
        let projection =
            |world_coords: [Vec3<f32>; 3]| RGBImage::screen_triangle_3d(world_coords, w, h);
        self.render_z_buffer_texture_projection(wireframe, texture, light_dir, &projection);
    }

    pub(crate) fn render_z_buffer_texture_perspective(
        &mut self,
        wireframe: WireframeModel,
        texture: DynamicImage,
        light_dir: Vec3<f32>,
    ) {
        let mut projection_matrix = Matrix::new_identity(4);
        let camera_z = 3.0;
        projection_matrix.m[3][2] = -1.0 / camera_z;
        let view_port = ViewPort {
            x: self.width / 8,
            y: self.height / 8,
            width: self.width * 3 / 4,
            height: self.height * 3 / 4,
        };
        let projection_viewport = view_port.to_matrix() * projection_matrix;
        let projection = |world_coords: [Vec3<f32>; 3]| {
            RGBImage::screen_triangle_3d_perspective(world_coords, projection_viewport.clone())
        };
        self.render_z_buffer_texture_projection(wireframe, texture, light_dir, &projection);
    }

    fn render_z_buffer_texture_projection(
        &mut self,
        wireframe: WireframeModel,
        texture: DynamicImage,
        light_dir: Vec3<f32>,
        projection: &dyn Fn([Vec3<f32>; 3]) -> [Vec3<u16>; 3],
    ) {
        let z_buffer_size: i32 = self.width as i32 * self.height as i32;
        let mut z_buffer: Vec<f32> = (0..z_buffer_size).map(|_x| -1.0).collect();

        for face in wireframe.faces {
            let world_coords = face.map(|f| wireframe.vertexes[f.vertex_index]);
            let mut n = cross(
                diff(world_coords[2], world_coords[0]),
                diff(world_coords[1], world_coords[0]),
            );
            n.normalize();
            let intensity = light_dir.x * n.x + light_dir.y * n.y + light_dir.z * n.z;

            let texture_coords = face
                .map(|f| wireframe.texture_coord[f.texture_index])
                .map(|p| Vec2 {
                    x: texture.width() as f32 * p.0,
                    y: texture.height() as f32 * p.1,
                });

            let texture_color = |bc: Vec3<f32>| -> RGBColor {
                let uv = Vec2 {
                    x: texture_coords[0].x * bc.x
                        + texture_coords[1].x * bc.y
                        + texture_coords[2].x * bc.z,
                    y: texture_coords[0].y * bc.x
                        + texture_coords[1].y * bc.y
                        + texture_coords[2].y * bc.z,
                };
                let pixel = texture.get_pixel(uv.x as u32, uv.y as u32);
                let color = RGBColor {
                    r: pixel.0[0],
                    g: pixel.0[1],
                    b: pixel.0[2],
                }
                .with_intensity(intensity);
                return color;
            };
            if intensity > 0.0 {
                let pts = projection(world_coords);
                self.triangle_z_buffer_bary(pts, &mut z_buffer, &texture_color);
            }
        }
    }
    fn screen_triangle(world_coords: [Vec3<f32>; 3], width: u16, height: u16) -> [Vec2<u16>; 3] {
        let projection = |world_coords: Vec3<f32>| Vec2::<u16> {
            x: ((world_coords.x + 1.0) * (width as f32) / 2.0) as u16,
            y: ((world_coords.y + 1.0) * (height as f32) / 2.0) as u16,
        };
        [
            projection(world_coords[0]),
            projection(world_coords[1]),
            projection(world_coords[2]),
        ]
    }

    fn screen_triangle_3d(world_coords: [Vec3<f32>; 3], width: u16, height: u16) -> [Vec3<u16>; 3] {
        let projection = |world_coords: Vec3<f32>| Vec3::<u16> {
            x: ((world_coords.x + 1.0) * (width as f32) / 2.0) as u16,
            y: ((world_coords.y + 1.0) * (height as f32) / 2.0) as u16,
            z: world_coords.z as u16,
        };
        [
            projection(world_coords[0]),
            projection(world_coords[1]),
            projection(world_coords[2]),
        ]
    }

    fn screen_triangle_3d_perspective(
        world_coords: [Vec3<f32>; 3],
        projection_matrix: Matrix,
    ) -> [Vec3<u16>; 3] {
        let projection = |world_coords: Vec3<f32>| {
            (projection_matrix.clone() * world_coords.to_matrix())
                .to_vector()
                .as_u16()
        };
        [
            projection(world_coords[0]),
            projection(world_coords[1]),
            projection(world_coords[2]),
        ]
    }
}

impl FromIterator<Point> for [Point; 3] {
    fn from_iter<T: IntoIterator<Item = Vec2<u16>>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        [it.next().unwrap(), it.next().unwrap(), it.next().unwrap()]
    }
}
