use std::ops::Add;

use glam::Vec3;

#[derive(Clone, Debug)]
pub struct Rotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vec3> for Rotation {
    fn from(value: Vec3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Rotation> for Vec3 {
    fn from(value: Rotation) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl Add for Rotation {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<Vec3> for Rotation {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

#[allow(dead_code)]
pub struct MovementInput {
    move_forward: f32,
    move_left: f32,
    sneak: bool,
    sprint: bool,
}

