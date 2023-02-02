use crate::geometry2d::point::Point;
use std::fmt::{Display, Formatter};

pub struct Line {
    pub p1: Point,
    pub p2: Point,
    pub length: f32,
}

impl Line {
    pub fn new(point1: &Point, point2: &Point) -> Self {
        Line {
            length: point1.distance(&point2),
            p1: point1.clone(),
            p2: point2.clone(),
        }
    }

    pub fn slope(&self) -> f32 {
        (self.p1.y - self.p2.y) / (self.p1.x - self.p2.x)
    }

    pub fn slope_negative_inverse(&self) -> f32 {
        (-(self.p1.x - self.p2.x)) / (self.p1.y - self.p2.y)
    }

    pub fn is_parallel(&self, other: &Self) -> bool {
        self.slope() == other.slope()
    }

    pub fn is_perpendicular(&self, other: Self) -> bool {
        self.slope_negative_inverse() == other.slope()
    }
}

impl Clone for Line {
    fn clone(&self) -> Self {
        Line::new(&self.p1, &self.p2)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<--->{}, D: {}", self.p1, self.p2, self.length)
    }
}
