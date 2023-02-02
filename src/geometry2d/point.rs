use std::fmt::{Display, format, Formatter};
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

    pub fn connect_points(points: &[Point]) -> Vec<Line> {
        let len = points.len();
        let mut lines: Vec<Line> = Vec::with_capacity(len);
        for i in 0..len - 1 {
            let p1 = &points[i];
            let p2 = &points[i + 1];
            lines.push(p1.as_line(p2));
        }

        let p1 = &points[len - 1];
        let p2 = &points[0];
        lines.push(p1.as_line(p2));
        lines
    }
}

impl Clone for Point {
    fn clone(&self) -> Self {
        Point::new(self.X, self.Y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.X, self.Y)
    }
}