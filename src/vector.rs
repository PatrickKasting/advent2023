use std::{f64, ops::Neg};

use itertools::Itertools;
use num_traits::{NumCast, NumOps, Zero};

pub trait Vector {
    type Scalar;
    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self::Scalar) -> Self;
    // fn div(self, rhs: Self::Scalar) -> Self;
    fn dot(self, rhs: Self) -> Self::Scalar;
}

impl<T, const N: usize> Vector for [T; N]
where
    T: Copy + Zero + NumOps,
{
    type Scalar = T;

    fn add(mut self, rhs: Self) -> Self {
        for (index, element) in self.iter_mut().enumerate() {
            *element = *element + rhs[index];
        }
        self
    }

    fn sub(mut self, rhs: Self) -> Self {
        for (index, element) in self.iter_mut().enumerate() {
            *element = *element - rhs[index];
        }
        self
    }

    fn mul(mut self, rhs: Self::Scalar) -> Self {
        for element in &mut self {
            *element = *element * rhs;
        }
        self
    }

    // fn div(mut self, rhs: Self::Scalar) -> Self {
    //     for element in &mut self {
    //         *element = *element / rhs;
    //     }
    //     self
    // }

    fn dot(self, rhs: Self) -> Self::Scalar {
        self.into_iter()
            .zip(rhs)
            .fold(T::zero(), |sum, (left, right)| sum + left * right)
    }
}

pub trait Negation {
    fn neg(self) -> Self;
}

impl<T, const N: usize> Negation for [T; N]
where
    T: Neg<Output = T>,
{
    fn neg(self) -> Self {
        self.map(|element| -element)
    }
}

pub trait Unit {
    fn unit(self) -> Self;
}

impl<const N: usize> Unit for [isize; N] {
    fn unit(self) -> Self {
        let (non_zero_index, element) = self
            .into_iter()
            .enumerate()
            .filter(|&(_, element)| element != 0)
            .exactly_one()
            .expect("exactly one element should be non-zero");

        let mut unit = [0; N];
        unit[non_zero_index] = element.signum();
        unit
    }
}

pub trait CrossProduct {
    fn cross(self, rhs: Self) -> Self;
}

impl<T> CrossProduct for [T; 3]
where
    T: Copy + NumOps,
{
    fn cross(self, rhs: Self) -> Self {
        [
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ]
    }
}

pub trait RotationInTwoDimensions {
    fn left(self) -> Self;
    fn right(self) -> Self;
}

impl<T> RotationInTwoDimensions for [T; 2]
where
    T: Neg<Output = T>,
{
    fn left(self) -> Self {
        let [x, y] = self;
        [-y, x]
    }

    fn right(self) -> Self {
        let [x, y] = self;
        [y, -x]
    }
}

pub trait AngleInTwoDimensions {
    fn angle(self, to: Self) -> f64;
}

impl<T> AngleInTwoDimensions for [T; 2]
where
    T: Copy + Zero + NumOps + NumCast,
{
    fn angle(self, to: Self) -> f64 {
        let dot = <f64 as NumCast>::from(self.dot(to)).expect("number should convert to 'f64'");
        let determinant = <f64 as NumCast>::from(self[0] * to[1] - self[1] * to[0])
            .expect("number should convert to 'f64'");
        determinant.atan2(dot)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::*;

    use crate::floating::ApproxEq;

    use super::*;

    #[test]
    fn addition() {
        let actual = [1, 2, 3].add([-6, -4, -2]);
        assert_eq!(actual, [-5, -2, 1]);
    }

    #[test]
    fn subtraction() {
        let actual = [1, 2, 3].sub([-6, -4, -2]);
        assert_eq!(actual, [7, 6, 5]);
    }

    #[test]
    fn scalar_multiplication() {
        let actual = [-3, 4, 0, -3, 2].mul(2);
        assert_eq!(actual, [-6, 8, 0, -6, 4]);
    }

    // #[test]
    // fn scalar_division() {
    //     let actual = [-4, 4, 0, -2, 8].div(2);
    //     assert_eq!(actual, [-2, 2, 0, -1, 4]);
    // }

    #[test]
    fn negation() {
        let vector = [-6, -4, -2];
        assert_eq!(vector.neg(), [6, 4, 2]);
    }

    #[test]
    fn dot_product() {
        let actual = [1, 3, -5, 2].dot([4, -2, -1, 2]);
        assert_eq!(actual, 7);
    }

    #[test]
    fn unit() {
        let actual = [0, -4, 0].unit();
        let expected = [0, -1, 0];
        assert_eq!(actual, expected);
    }

    #[test]
    fn cross() {
        let actual = [3, -3, 1].cross([4, 9, 2]);
        assert_eq!(actual, [-15, -2, 39]);
    }

    #[test]
    fn left() {
        let actual = [2, 1].left();
        assert_eq!(actual, [-1, 2]);
    }

    #[test]
    fn right() {
        let actual = [-1, 2].right();
        assert_eq!(actual, [2, 1]);
    }

    #[test]
    fn angle() {
        let cases = [
            ([[1, 0], [3, 0]], 0.0),
            ([[2, 0], [4, 4]], FRAC_PI_4),
            ([[3, 0], [0, 5]], FRAC_PI_2),
            ([[4, 0], [-1, 1]], 3.0 * FRAC_PI_4),
            ([[5, 0], [-2, 0]], PI),
            ([[6, 0], [-3, -3]], -3.0 * FRAC_PI_4),
            ([[7, 0], [0, -4]], -FRAC_PI_2),
            ([[8, 0], [5, -5]], -FRAC_PI_4),
        ];
        for ([from, to], expected) in cases {
            let actual = from.angle(to) / PI;
            let expected = expected / PI;
            assert!(
                actual.approx_eq(expected),
                "actual angle {actual:.2}π from {from:?} to {to:?} should equal {expected:.2}π"
            );
        }
    }
}
