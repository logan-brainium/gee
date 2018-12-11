use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    iter::{Product, Sum},
    marker::PhantomData,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign},
};

use crate::{op};

// TODO should forward all trait functions, not just those w/o defaults

#[derive(Debug)] // TODO don't derive Debug
pub struct Scalar<T, Unit> {
    t:    T,
    unit: PhantomData<Unit>,
}

impl<T, Unit> Scalar<T, Unit> {
    pub fn new(t: T) -> Self {
        Scalar {
            t,
            unit: PhantomData,
        }
    }
}

// TODO new implementation specialized for float-like types that'll assert not NaN/Infinity/...

impl<T: Default, Unit> Default for Scalar<T, Unit> {
    fn default() -> Self {
        T::default().into()
    }
}

impl<T: Hash, Unit> Hash for Scalar<T, Unit> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.t.hash(state)
    }
}

impl<T: Clone, Unit> Clone for Scalar<T, Unit> {
    fn clone(&self) -> Self {
        self.t.clone().into()
    }
}

impl<T: Copy, Unit> Copy for Scalar<T, Unit> {}

impl<T: PartialEq, Unit> Eq for Scalar<T, Unit> {}

impl<T: PartialEq, Unit> PartialEq for Scalar<T, Unit> {
    fn eq(&self, other: &Self) -> bool {
        self.t.eq(&other.t)
    }
}

impl<T: PartialOrd, Unit> PartialOrd for Scalar<T, Unit> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}

impl<T: PartialOrd, Unit> Ord for Scalar<T, Unit> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}

impl<T, Unit> From<T> for Scalar<T, Unit> {
    fn from(t: T) -> Self {
        Scalar {
            t,
            unit: PhantomData,
        }
    }
}

impl<T, Unit> AsRef<T> for Scalar<T, Unit> {
    fn as_ref(&self) -> &T {
        &self.t
    }
}

impl<T: Neg<Output = Output>, Unit, Output> Neg for Scalar<T, Unit> {
    type Output = Scalar<Output, Unit>;
    fn neg(self) -> Self::Output {
        self.t.neg().into()
    }
}

impl<T: Product<A>, Unit, A> Product<Scalar<A, Unit>> for Scalar<T, Unit> {
    fn product<I>(iter: I) -> Scalar<T, Unit>
    where
        I: Iterator<Item = Scalar<A, Unit>>,
    {
        T::product(iter.map(|x| x.t)).into()
    }
}

impl<T: Sum<A>, Unit, A> Sum<Scalar<A, Unit>> for Scalar<T, Unit> {
    fn sum<I>(iter: I) -> Scalar<T, Unit>
    where
        I: Iterator<Item = Scalar<A, Unit>>,
    {
        T::sum(iter.map(|x| x.t)).into()
    }
}

op_term!(Scalar, Add, add, AddAssign, add_assign, t);
op_term!(Scalar, Sub, sub, SubAssign, sub_assign, t);
op_factor!(Scalar, Rem, rem, RemAssign, rem_assign, t);
op_factor!(Scalar, Div, div, DivAssign, div_assign, t);
op_factor!(Scalar, Mul, mul, MulAssign, mul_assign, t);