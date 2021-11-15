#[derive(Clone, Copy)]
pub(crate) struct Point {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

impl Point {
    pub(crate) fn from(x: i32, y: i32) -> Point {
        assert!(x >= 0);
        assert!(y >= 0);
        Point {
            x: x as u16,
            y: y as u16,
        }
    }
    pub(crate) fn shift(self, dx: i32, dy: i32) -> Point {
        Point::from(self.x as i32 + dx, self.y as i32 + dy)
    }
}
