use crate::geometry2d::line::Line;

pub struct Point {
    pub X: f32,
    pub Y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Point { X: x, Y: y }
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let xx = (self.X - other.X) * (self.X - other.X);
        let yy = (self.Y - other.Y) * (self.Y - other.Y);
        (xx + yy).sqrt()
    }

    pub fn as_line(&self, other: &Point) -> Line {
        Line::new(&self, &other)
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point::new(self.X, self.Y)
    }
}
