use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Point2<T> {
    x: T,
    y: T,
}

impl<T> Point2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display> Display for Point2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

macro_rules! point2_impl {
    ($t:ty) => {
        impl Point2<$t> {
            #[inline]
            pub const fn zero() -> Self {
                Self::new(0, 0)
            }

            #[inline]
            pub const fn add(self, other: Self) -> Self {
                Self::new(self.x + other.x, self.y + other.y)
            }

            #[inline]
            pub const fn scale(self, f: $t) -> Self {
                Self::new(self.x * f, self.y * f)
            }

            #[inline]
            pub fn neighbors4(self) -> [Self; 4] {
                Self::dirs4().map(|d| self + d)
            }

            #[inline]
            pub fn neighbors8(self) -> [Self; 8] {
                Self::dirs8().map(|d| self + d)
            }

            pub const fn dirs4() -> [Self; 4] {
                [
                    Self::new(-1, 0),
                    Self::new(0, -1),
                    Self::new(1, 0),
                    Self::new(0, 1),
                ]
            }

            pub const fn dirs8() -> [Self; 8] {
                [
                    Self::new(-1, -1),
                    Self::new(-1, 0),
                    Self::new(-1, 1),
                    Self::new(0, -1),
                    Self::new(0, 1),
                    Self::new(1, -1),
                    Self::new(1, 0),
                    Self::new(1, 1),
                ]
            }
        }

        impl Add for Point2<$t> {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                self.add(other)
            }
        }

        impl AddAssign<Point2<$t>> for Point2<$t> {
            fn add_assign(&mut self, other: Self) {
                self.x += other.x;
                self.y += other.y;
            }
        }

        impl Mul<$t> for Point2<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> Self {
                self.scale(other)
            }
        }

        impl MulAssign<$t> for Point2<$t> {
            fn mul_assign(&mut self, other: $t) {
                self.x *= other;
                self.y *= other;
            }
        }
    };
}

point2_impl!(i8);
point2_impl!(i16);
point2_impl!(i32);
point2_impl!(i64);
point2_impl!(isize);
