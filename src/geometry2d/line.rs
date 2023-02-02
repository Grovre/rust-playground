use std::fmt::{Display, Formatter};
use crate::geometry2d::point::Point;

pub struct Line {
    pub P1: Point,
    pub P2: Point,
    pub Length: f32,
}

impl Line {
    pub fn new(point1: &Point, point2: &Point) -> Self {
        Line {
            Length: point1.distance(&point2),
            P1: point1.clone(),
            P2: point2.clone(),
        }
    }

    pub fn slope(&self) -> f32 {
        (self.P1.Y - self.P2.Y) / (self.P1.X - self.P2.X)
    }

    pub fn slope_negative_inverse(&self) -> f32 {
        (-(self.P1.X - self.P2.X)) / (self.P1.Y - self.P2.Y)
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
        Line::new(&self.P1, &self.P2)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}<--->{}, D: {}", self.P1, self.P2, self.Length)
    }
}