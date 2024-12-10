use std::ops::{Add, Mul, Sub};

pub type Vec2Coord = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2([Vec2Coord; 2]);
impl Vec2 {
    pub fn x(&self) -> Vec2Coord {
        self.0[0]
    }

    pub fn y(&self) -> Vec2Coord {
        self.0[1]
    }
}
impl From<[Vec2Coord; 2]> for Vec2 {
    fn from(value: [Vec2Coord; 2]) -> Self {
        Self(value)
    }
}
impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::from([self.x() - rhs.x(), self.y() - rhs.y()])
    }
}
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from([self.x() + rhs.x(), self.y() + rhs.y()])
    }
}
impl Mul<Vec2Coord> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2Coord) -> Self::Output {
        Self::from([self.x() * rhs, self.y() * rhs])
    }
}
