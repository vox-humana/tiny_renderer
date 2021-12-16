use crate::matrix::{look_at, Matrix, ViewPort};
use crate::point::{cross, diff, dot_product, Point, Vec2, Vec3};
use crate::rgb_image::{RGBColor, RGBImage, BLACK_COLOR};
use crate::wireframe::{Face, WireframeModel};
use image::{DynamicImage, GenericImageView};

impl RGBImage {
    pub(crate) fn render_frame(&mut self, wireframe: WireframeModel, color: RGBColor) {
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

    pub(crate) fn render_random(&mut self, wireframe: WireframeModel) {
        for face in wireframe.faces {
            let world_coords = face.map(|f| wireframe.vertexes[f.vertex_index]);
            let pts = RGBImage::screen_triangle(world_coords, self.width, self.height);
            self.triangle_filed(pts, RGBColor::random());
        }
    }

    pub(crate) fn render_light(&mut self, wireframe: WireframeModel, light_dir: Vec3<f32>) {
        for face in wireframe.faces {
            let world_coords = face.map(|f| wireframe.vertexes[f.vertex_index]);
            let n = cross(
                diff(world_coords[2], world_coords[0]),
                diff(world_coords[1], world_coords[0]),
            )
            .normalized();
            let intensity = dot_product(light_dir, n);

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
            let n = cross(
                diff(world_coords[2], world_coords[0]),
                diff(world_coords[1], world_coords[0]),
            )
            .normalized();
            let intensity = dot_product(light_dir, n);

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

        for face in &wireframe.faces {
            let world_coords = face.map(|f| wireframe.vertexes[f.vertex_index]);
            let n = cross(
                diff(world_coords[2], world_coords[0]),
                diff(world_coords[1], world_coords[0]),
            )
            .normalized();
            let intensity = dot_product(light_dir, n);

            let texture_coords =
                wireframe.texture_coord(face.clone(), texture.width(), texture.height());
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

    pub(crate) fn render_z_buffer_texture_perspective_gouraud(
        &mut self,
        wireframe: WireframeModel,
        texture: DynamicImage,
        light_dir: Vec3<f32>,
    ) {
        let eye = Vec3 {
            x: 1.0,
            y: 1.0,
            z: 3.0,
        };
        let center = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let model_view = look_at(
            eye,
            center,
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        );
        let mut projection_matrix = Matrix::new_identity(4);
        let camera_z = 3.0;
        projection_matrix.m[3][2] = -1.0 / camera_z;
        let view_port = ViewPort {
            x: self.width / 8,
            y: self.height / 8,
            width: self.width * 3 / 4,
            height: self.height * 3 / 4,
        };

        let z = view_port.to_matrix() * projection_matrix * model_view;
        let projection = |world_coords: [Vec3<f32>; 3]| {
            RGBImage::screen_triangle_3d_perspective(world_coords, z.clone())
        };

        let z_buffer_size: i32 = self.width as i32 * self.height as i32;
        let mut z_buffer: Vec<f32> = (0..z_buffer_size).map(|_x| -1.0).collect();

        for face in &wireframe.faces {
            let world_coords = face.map(|f| wireframe.vertexes[f.vertex_index]);
            let intensity = face
                .map(|f| wireframe.norm[f.norm_index])
                .map(|mut n: Vec3<f32>| {
                    n.normalize();
                    return light_dir.x * n.x + light_dir.y * n.y + light_dir.z * n.z;
                });

            let texture_coords =
                wireframe.texture_coord(face.clone(), texture.width(), texture.height());
            let texture_color = |bc: Vec3<f32>| -> RGBColor {
                let uv = Vec2 {
                    x: texture_coords[0].x * bc.x
                        + texture_coords[1].x * bc.y
                        + texture_coords[2].x * bc.z,
                    y: texture_coords[0].y * bc.x
                        + texture_coords[1].y * bc.y
                        + texture_coords[2].y * bc.z,
                };
                let weighted_intensity =
                    intensity[0] * bc.x + intensity[1] * bc.y + intensity[2] * bc.z;

                let pixel = texture.get_pixel(uv.x as u32, uv.y as u32);
                let color = RGBColor {
                    r: pixel.0[0],
                    g: pixel.0[1],
                    b: pixel.0[2],
                };

                return if weighted_intensity > 0.0 {
                    color.with_intensity(weighted_intensity)
                } else {
                    BLACK_COLOR
                };
            };
            let pts = projection(world_coords);
            self.triangle_z_buffer_bary(pts, &mut z_buffer, &texture_color);
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

impl WireframeModel {
    fn texture_coord(&self, face: [Face; 3], width: u32, height: u32) -> [Vec2<f32>; 3] {
        face.map(|f| self.texture_coord[f.texture_index])
            .map(|p| Vec2 {
                x: width as f32 * p.0,
                y: height as f32 * p.1,
            })
    }
}
