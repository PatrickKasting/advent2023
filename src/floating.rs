pub trait ApproxEq {
    fn approx_eq(self, rhs: Self) -> bool;
}

impl ApproxEq for f64 {
    fn approx_eq(self, rhs: Self) -> bool {
        (self - rhs).abs() < Self::EPSILON
    }
}
