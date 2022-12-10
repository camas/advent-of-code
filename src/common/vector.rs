use std::ops::{AddAssign, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2 {
    pub x: i64,
    pub y: i64,
}

impl Vector2 {
    pub fn new(x: i64, y: i64) -> Vector2 {
        Vector2 { x, y }
    }
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
