use crate::point::Point;
use crate::rgb_image::{RGBImage, BLACK_COLOR, GREEN_COLOR, RED_COLOR, WHITE_COLOR};
use crate::wireframe::WireframeModel;

fn lesson0() -> RGBImage {
    let mut image = RGBImage::new(100, 100, BLACK_COLOR);
    image.set_pixel(Point { x: 10, y: 80 }, RED_COLOR);
    return image;
}

fn lesson1() -> RGBImage {
    let mut image = RGBImage::new(100, 100, BLACK_COLOR);
    image.line(Point { x: 13, y: 20 }, Point { x: 80, y: 40 }, WHITE_COLOR);
    image.line(Point { x: 20, y: 13 }, Point { x: 40, y: 80 }, RED_COLOR);
    image.line(Point { x: 40, y: 80 }, Point { x: 13, y: 20 }, GREEN_COLOR);
    return image;
}

fn lesson1_1() -> RGBImage {
    let mut image = RGBImage::new(640, 640, BLACK_COLOR);
    let model = WireframeModel::from_file("african_head.obj".to_string());
    image.render(model, WHITE_COLOR);
    image.flip_vertically();
    return image;
}

fn lesson2() -> RGBImage {
    let mut image = RGBImage::new(640, 640, BLACK_COLOR);
    let t0 = [(10, 70), (50, 160), (70, 80)];
    let t1 = [(180, 50), (150, 1), (70, 180)];
    let t2 = [(180, 150), (120, 160), (130, 180)];
    let points = |a: [(u16, u16); 3]| a.map(|t| Point { x: t.0, y: t.1 });
    image.triangle_v(points(t0), RED_COLOR);
    image.triangle_v(points(t1), WHITE_COLOR);
    image.triangle_v(points(t2), GREEN_COLOR);

    let shift = |points: [Point; 3], dx: i32, dy: i32| points.map(|p| p.shift(dx, dy));

    image.triangle_v_sorted(shift(points(t0), 300, 0));
    image.triangle_v_sorted(shift(points(t1), 300, 0));
    image.triangle_v_sorted(shift(points(t2), 300, 0));

    image.triangle_filed(shift(points(t0), 0, 300), RED_COLOR);
    image.triangle_filed(shift(points(t1), 0, 300), WHITE_COLOR);
    image.triangle_filed(shift(points(t2), 0, 300), GREEN_COLOR);

    image.flip_vertically();
    return image;
}

#[derive(Copy, Clone)]
pub struct Lesson {
    pub name: &'static str,
    pub renderer: fn() -> RGBImage,
}

impl Lesson {
    pub fn same(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn lessons() -> [Lesson; 4] {
    [
        Lesson {
            name: "Pixel",
            renderer: lesson0,
        },
        Lesson {
            name: "Bresenham",
            renderer: lesson1,
        },
        Lesson {
            name: "Wireframe",
            renderer: lesson1_1,
        },
        Lesson {
            name: "Triangles",
            renderer: lesson2,
        },
    ]
}
