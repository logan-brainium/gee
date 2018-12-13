use crate::vector::Vector;
use std::ops::Add;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Size<T> {
    vector: Vector<T>,
}

impl<T> Size<T> {
    pub fn new(width: T, height: T) -> Self {
        Size {
            vector: Vector::new(width, height),
        }
    }

    pub fn width(&self) -> &T {
        &self.vector.dx
    }

    pub fn height(&self) -> &T {
        &self.vector.dy
    }
}

impl<T: Add<RHS, Output = Output>, RHS, Output> Add<Size<RHS>> for Size<T> {
    type Output = Size<Output>;
    fn add(self, rhs: Size<RHS>) -> Self::Output {
        Size {
            vector: self.vector + rhs.vector,
        }
    }
}
