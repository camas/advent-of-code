#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Vector3 {
    pub fn new(x: i64, y: i64, z: i64) -> Vector3 {
        Vector3 { x, y, z }
    }

    pub fn rotate_90_around_x(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: -self.z,
            z: self.y,
        }
    }

    pub fn rotate_270_around_x(&self) -> Vector3 {
        Vector3 {
            x: self.x,
            y: self.z,
            z: -self.y,
        }
    }

    pub fn rotate_90_around_y(&self) -> Vector3 {
        Vector3 {
            x: self.z,
            y: self.y,
            z: -self.x,
        }
    }

    pub fn rotate_270_around_y(&self) -> Vector3 {
        Vector3 {
            x: -self.z,
            y: self.y,
            z: self.x,
        }
    }

    pub fn rotate_90_around_z(&self) -> Vector3 {
        Vector3 {
            x: self.y,
            y: -self.x,
            z: self.z,
        }
    }

    pub fn rotate_270_around_z(&self) -> Vector3 {
        Vector3 {
            x: -self.y,
            y: self.x,
            z: self.z,
        }
    }
}
