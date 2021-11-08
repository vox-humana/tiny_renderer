use crate::rgb_image;
use crate::rgb_image::{Point, RGBImage};
use crate::wireframe::WireframeModel;

fn lesson0() -> RGBImage {
    let mut image = RGBImage::new(100, 100, rgb_image::BLACK_COLOR);
    image.set_pixel(Point{ x: 10, y: 80 }, rgb_image::RED_COLOR);
    return image;
}

fn lesson1() -> RGBImage {
    let mut image = RGBImage::new(100, 100, rgb_image::BLACK_COLOR);
    image.line(Point{ x: 13, y: 20 }, Point{ x: 80, y: 40 }, rgb_image::WHITE_COLOR);
    image.line(Point{ x: 20, y: 13 }, Point{ x: 40, y: 80 }, rgb_image::RED_COLOR);
    image.line(Point{ x: 40, y: 80 }, Point{ x: 13, y: 20 }, rgb_image::GREEN_COLOR);
    return image;
}


fn lesson1_1() -> RGBImage {
    let mut image = RGBImage::new(640, 640, rgb_image::BLACK_COLOR);
    let model = WireframeModel::from_file("african_head.obj".to_string());
    image.render(model, rgb_image::WHITE_COLOR);
    image.flip();
    return image;
}

#[derive(Copy, Clone)]
pub struct Lesson {
    pub name: &'static str,
    pub renderer: fn () -> RGBImage,
}

impl Lesson {
    pub fn same(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn lessons() -> [Lesson; 3] {
    [
        Lesson{ name: "Pixel", renderer: lesson0 },
        Lesson{ name: "Bresenham", renderer: lesson1 },
        Lesson{ name: "Wireframe", renderer: lesson1_1 }
    ]
}