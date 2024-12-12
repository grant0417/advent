use std::hash::Hash;
use std::num::TryFromIntError;

pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn map<J>(self, f: impl Fn(T) -> J) -> Point<J> {
        Point {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

impl<T> From<(T, T)> for Point<T> {
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> Clone for Point<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

impl<T> Copy for Point<T> where T: Copy {}

impl<T> std::fmt::Debug for Point<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Point")
            .field(&self.x)
            .field(&self.y)
            .finish()
    }
}

impl<T> PartialEq for Point<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl<T> Eq for Point<T> where T: Eq {}

impl<T> Hash for Point<T>
where
    T: Hash,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl<T> PartialOrd for Point<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(
            self.x
                .partial_cmp(&other.x)?
                .then(self.y.partial_cmp(&other.y)?),
        )
    }
}

impl<T> Ord for Point<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

macro_rules! impl_ops {
    ($t: ty) => {
        impl Point<$t> {
            pub fn scale(&self, v: $t) -> Self {
                Self {
                    x: self.x * v,
                    y: self.y * v,
                }
            }

            pub fn manhattan_distance(&self, other: &Self) -> $t {
                (self.x as i64 - other.x as i64).unsigned_abs() as $t
                    + (self.y as i64 - other.y as i64).unsigned_abs() as $t
            }
        }

        impl std::ops::Add for Point<$t> {
            type Output = Self;

            fn add(self, other: Self) -> Self::Output {
                Self {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        impl std::ops::AddAssign for Point<$t> {
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }

        impl std::ops::Sub for Point<$t> {
            type Output = Self;

            fn sub(self, other: Self) -> Self::Output {
                Self {
                    x: self.x - other.x,
                    y: self.y - other.y,
                }
            }
        }

        impl std::ops::SubAssign for Point<$t> {
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }
    };
}

impl_ops!(u8);
impl_ops!(u16);
impl_ops!(u32);
impl_ops!(u64);
impl_ops!(usize);
impl_ops!(i8);
impl_ops!(i16);
impl_ops!(i32);
impl_ops!(i64);
impl_ops!(isize);

impl TryInto<Point<usize>> for Point<i64> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Point<usize>, Self::Error> {
        Ok(Point {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
        })
    }
}

impl TryInto<Point<usize>> for (i64, i64) {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Point<usize>, Self::Error> {
        Ok(Point {
            x: self.0.try_into()?,
            y: self.1.try_into()?,
        })
    }
}

impl TryInto<Point<i64>> for Point<usize> {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Point<i64>, Self::Error> {
        Ok(Point {
            x: self.x.try_into()?,
            y: self.y.try_into()?,
        })
    }
}

impl TryInto<Point<i64>> for (usize, usize) {
    type Error = TryFromIntError;

    fn try_into(self) -> Result<Point<i64>, Self::Error> {
        Ok(Point {
            x: self.0.try_into()?,
            y: self.1.try_into()?,
        })
    }
}
