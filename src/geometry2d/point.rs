pub struct Point {
    pub X: f32,
    pub Y: f32
}

impl Point {
    pub fn distance(&self, other: &Self) -> f32 {
        let xx = (self.X - other.X) * (self.X - other.X);
        let yy = (self.Y - other.Y) * (self.Y - other.Y);
        (xx + yy).sqrt()
    }
}