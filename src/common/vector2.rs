use std::ops::{Add, AddAssign, Sub};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vector2 {
    pub x: i64,
    pub y: i64,
}

impl Vector2 {
    pub const fn new(x: i64, y: i64) -> Vector2 {
        Vector2 { x, y }
    }

    pub fn manhattan_distance(&self, other: Vector2) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn add_y(&self, y: i64) -> Vector2 {
        Vector2::new(self.x, self.y + y)
    }

    pub fn add_x(&self, x: i64) -> Vector2 {
        Vector2::new(self.x + x, self.y)
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a> Add for &'a Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Self) -> Self::Output {
        (*self).add(*rhs)
    }
}

impl<'a> Add<&'a Vector2> for Vector2 {
    type Output = Vector2;

    fn add(self, rhs: &'a Vector2) -> Self::Output {
        self.add(*rhs)
    }
}

impl<'a> Add<Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn add(self, rhs: Vector2) -> Self::Output {
        (*self).add(rhs)
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

impl<'a> Sub for &'a Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Self) -> Self::Output {
        (*self).sub(*rhs)
    }
}

impl<'a> Sub<&'a Vector2> for Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: &'a Vector2) -> Self::Output {
        self.sub(*rhs)
    }
}

impl<'a> Sub<Vector2> for &'a Vector2 {
    type Output = Vector2;

    fn sub(self, rhs: Vector2) -> Self::Output {
        (*self).sub(rhs)
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
