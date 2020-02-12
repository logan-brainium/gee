use crate::{OrdinaryFloat, OrdinaryNum, Point, Ray, Vec2};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(C)]
pub struct LineSegment<T> {
    pub from: Point<T>,
    pub to:   Point<T>,
}

impl<T: OrdinaryNum> LineSegment<T> {
    pub fn new(from: Point<T>, to: Point<T>) -> Self {
        Self { from, to }
    }

    pub fn length(&self) -> T
    where
        T: OrdinaryFloat,
    {
        (self.to - self.from).magnitude()
    }

    pub fn vec2(&self) -> Vec2<T> {
        self.to - self.from
    }

    pub fn ray(&self) -> Ray<T>
    where
        T: OrdinaryFloat,
    {
        Ray::new(self.from, self.vec2().angle())
    }
}
