use num_traits::{One, Zero};
use std::cmp::Eq;
use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Field:
    Clone
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + Eq
    + Zero
    + One
{
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct F<T>(pub T)
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One;

impl<T: fmt::Debug> fmt::Debug for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let F(x) = self;
        x.fmt(f)
    }
}

impl<T: fmt::Display> fmt::Display for F<T>
where
    T:  Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let F(x) = self;
        x.fmt(f)
    }
}

impl<T> Add<Self> for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let F(x) = self;
        let F(y) = other;
        F(x + y)
    }
}

impl<T> AddAssign for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    fn add_assign(&mut self, other: Self) {
        *self = self.clone() + other;
    }
}

impl<T> Sub<Self> for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let F(x) = self;
        let F(y) = other;
        F(x - y)
    }
}

impl<T> SubAssign for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    fn sub_assign(&mut self, other: Self) {
        *self = self.clone() - other;
    }
}

impl<T> Neg for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    type Output = Self;
    fn neg(self) -> Self {
        Self::zero() - self
    }
}

impl<T> Mul<Self> for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let F(x) = self;
        let F(y) = other;
        F(x * y)
    }
}

impl<T> MulAssign for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    fn mul_assign(&mut self, other: Self) {
        *self = self.clone() * other;
    }
}

impl<T> Div<Self> for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let F(x) = self;
        let F(y) = other;
        F(x / y)
    }
}

impl<T> DivAssign for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    fn div_assign(&mut self, other: Self) {
        *self = self.clone() / other;
    }
}

impl<T> Zero for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    fn zero() -> Self {
        F(T::zero())
    }

    fn is_zero(&self) -> bool {
        let F(x) = self;
        x.is_zero()
    }
}

impl<T> One for F<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Eq
        + Zero
        + One,
{
    fn one() -> Self {
        F(T::one())
    }
}

impl <T> Field for T
where
    T: Clone
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Neg<Output = Self>
    + Eq
    + Zero
    + One
{
}
