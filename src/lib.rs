use std::ops::Mul;

use bevy::prelude::Vec3;

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug, Eq, Ord)]
pub struct Vector<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn rotate(&self, angle: f64) -> Vector<T>
    where
        T: Into<f64> + From<f64> + Copy,
    {
        let x = self.x.into();
        let y = self.y.into();

        let new_x = (x * angle.cos() - y * angle.sin()).into();
        let new_y = (x * angle.sin() + y * angle.cos()).into();

        Vector::new(new_x, new_y)
    }

    pub fn angle(&self) -> f64
    where
        T: Into<f64> + From<f64> + Copy,
    {
        let x = self.x.into();
        let y = self.y.into();
        y.atan2(x)
    }

    pub fn orthogonal(&self, dir: Direction) -> Self
    where
        T: std::ops::Neg<Output = T> + Copy,
    {
        match dir {
            Direction::Right => Vector::new(self.y, -self.x),
            Direction::Left => Vector::new(-self.y, self.x),
            _ => panic!("Invalid direction"),
        }
    }

    pub fn dot(&self, rhs: Self) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
    {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Vector<f64> {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Vector<f64> {
        let magnitude = self.magnitude();
        if magnitude > 0.0 {
            Vector { x: self.x / magnitude, y: self.y / magnitude }
        } else {
            Vector { x: self.x, y: self.y }
        }
    }
}

impl<T> std::ops::Add for Vector<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> std::ops::Sub for Vector<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> std::ops::AddAssign for Vector<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> std::ops::AddAssign<T> for Vector<T>
where
    T: std::ops::AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: T) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl<T> std::ops::MulAssign for Vector<T>
where
    T: std::ops::MulAssign,
{
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T> std::ops::SubAssign for Vector<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> std::ops::SubAssign<T> for Vector<T>
where
    T: std::ops::SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: T) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl<T> std::ops::Mul<T> for Vector<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T> std::ops::MulAssign<T> for Vector<T>
where
    T: std::ops::MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl From<Vector<f64>> for Vector<i32> {
    fn from(v: Vector<f64>) -> Self {
        Self {
            x: v.x.round() as i32,
            y: v.y.round() as i32,
        }
    }
}

impl From<Vector<i32>> for Vector<f64> {
    fn from(v: Vector<i32>) -> Self {
        Self {
            x: v.x as f64,
            y: v.y as f64,
        }
    }
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// ----------------------------

pub mod consts {
    pub const G: f32 = 6.67430e-11;
}
