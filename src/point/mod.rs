use std::fmt::Display;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub enum Heading {
    North,
    East,
    South,
    West,
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
            pub const fn dist_squared(self, other: Self) -> $t {
                let dx = self.x - other.x;
                let dy = self.y - other.y;
                dx * dx + dy * dy
            }

            #[inline]
            pub fn dist(self, other: Self) -> f64 {
                (self.dist_squared(other) as f64).sqrt()
            }

            #[inline]
            pub const fn step(self, heading: Heading) -> Self {
                self.add(Self::unit(heading))
            }

            #[inline]
            pub const fn unit(heading: Heading) -> Self {
                match heading {
                    Heading::North => Self::new(0, -1),
                    Heading::East => Self::new(1, 0),
                    Heading::South => Self::new(0, 1),
                    Heading::West => Self::new(-1, 0),
                }
            }

            #[inline]
            pub const fn left(self) -> Self {
                self.step(Heading::West)
            }

            #[inline]
            pub const fn right(self) -> Self {
                self.step(Heading::East)
            }

            #[inline]
            pub const fn up(self) -> Self {
                self.step(Heading::North)
            }

            #[inline]
            pub const fn down(self) -> Self {
                self.step(Heading::South)
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
                    Self::unit(Heading::North),
                    Self::unit(Heading::East),
                    Self::unit(Heading::South),
                    Self::unit(Heading::West),
                ]
            }

            pub const fn dirs8() -> [Self; 8] {
                [
                    Self::unit(Heading::North),
                    Self::unit(Heading::North).add(Self::unit(Heading::East)),
                    Self::unit(Heading::East),
                    Self::unit(Heading::East).add(Self::unit(Heading::South)),
                    Self::unit(Heading::South),
                    Self::unit(Heading::South).add(Self::unit(Heading::West)),
                    Self::unit(Heading::West),
                    Self::unit(Heading::West).add(Self::unit(Heading::North)),
                ]
            }
        }

        impl From<Heading> for Point2<$t> {
            fn from(heading: Heading) -> Self {
                Self::unit(heading)
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

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Display> Display for Point3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

macro_rules! point3_impl {
    ($t:ty) => {
        impl Point3<$t> {
            #[inline]
            pub const fn zero() -> Self {
                Self::new(0, 0, 0)
            }

            #[inline]
            pub const fn add(self, other: Self) -> Self {
                Self::new(self.x + other.x, self.y + other.y, self.z + self.z)
            }

            #[inline]
            pub const fn scale(self, f: $t) -> Self {
                Self::new(self.x * f, self.y * f, self.z * f)
            }

            #[inline]
            pub const fn dist_squared(self, other: Self) -> $t {
                let dx = self.x - other.x;
                let dy = self.y - other.y;
                let dz = self.z - other.z;
                dx * dx + dy * dy + dz * dz
            }

            #[inline]
            pub fn dist(self, other: Self) -> f64 {
                (self.dist_squared(other) as f64).sqrt()
            }
        }

        impl Add for Point3<$t> {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                self.add(other)
            }
        }

        impl AddAssign<Point3<$t>> for Point3<$t> {
            fn add_assign(&mut self, other: Self) {
                self.x += other.x;
                self.y += other.y;
                self.z += other.z;
            }
        }

        impl Mul<$t> for Point3<$t> {
            type Output = Self;

            fn mul(self, other: $t) -> Self {
                self.scale(other)
            }
        }

        impl MulAssign<$t> for Point3<$t> {
            fn mul_assign(&mut self, other: $t) {
                self.x *= other;
                self.y *= other;
                self.z *= other;
            }
        }
    };
}

point3_impl!(i8);
point3_impl!(i16);
point3_impl!(i32);
point3_impl!(i64);
point3_impl!(isize);
