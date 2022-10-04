use std::ops::{Neg, Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
pub(crate) struct Value(f64);

pub(crate) fn print_value(val: &Value) {
    print!("'{}'\n", val.0);
}

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Value(-self.0)
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Value(self.0 + rhs.0)
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Value(self.0 - rhs.0)
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Value(self.0 * rhs.0)
    }
}

impl Div for Value {
    type Output = Value;

    fn div(self, rhs: Self) -> Self::Output {
        Value(self.0 / rhs.0)
    }
}

impl Value {
    pub(crate) fn new(val: f64) -> Self {
        Self(val)
    }
}