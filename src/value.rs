use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ptr::write;

use crate::Bytecodes;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum ValueKind {
    Nil,
    Bool,
    Number,
    String,
    Function,
}

#[derive(Debug, Clone)]
pub enum ValueRepr {
    Nil(),
    Boolean(bool),
    Number(f64),
    String(String),
}

#[derive(Debug, Clone)]
pub struct Value(pub ValueRepr);

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.0 {
            ValueRepr::Boolean(v) => Value(ValueRepr::Boolean(!v)),
            ValueRepr::Number(v) => Value(ValueRepr::Number(-v)),
            _ => Value(ValueRepr::Nil()),
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => Value(ValueRepr::Number(l + r)),
            (ValueRepr::String(l), ValueRepr::String(r)) => Value(ValueRepr::String(l + &r)),
            _ => Value(ValueRepr::Nil()),
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => Value(ValueRepr::Number(l - r)),
            _ => Value(ValueRepr::Nil()),
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => Value(ValueRepr::Number(l * r)),
            (ValueRepr::String(l), ValueRepr::Number(t)) => Value(ValueRepr::String(l.repeat(t as usize))),
            _ => Value(ValueRepr::Nil()),
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => Value(ValueRepr::Number(l / r)),
            _ => Value(ValueRepr::Nil()),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => l == r,
            (ValueRepr::Boolean(l), ValueRepr::Boolean(r)) => l == r,
            (ValueRepr::Nil(), ValueRepr::Nil()) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (&self.0, &other.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => {
                if l < r {
                    return Some(Ordering::Less);
                } else if l > r {
                    return Some(Ordering::Greater);
                }
                return Some(Ordering::Equal);
            }
            (ValueRepr::String(l), ValueRepr::String(r)) => {
                if l < r {
                    return Some(Ordering::Less);
                } else if l > r {
                    return Some(Ordering::Greater);
                }
                return Some(Ordering::Equal);
            }
            _ => None,
        }
    }
}

impl ValueRepr {
    pub fn kind(&self) -> ValueKind {
        match &self {
            ValueRepr::Boolean(_) => ValueKind::Bool,
            ValueRepr::Number(_) => ValueKind::Number,
            ValueRepr::String(_) => ValueKind::String,
            ValueRepr::Nil() => ValueKind::Nil,
        }
    }
}

impl Default for ValueRepr {
    fn default() -> ValueRepr {
        ValueRepr::Nil().into()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ValueRepr::Boolean(val) => write!(f, "{}", val),
            ValueRepr::Number(val) => write!(f, "{}", val),
            ValueRepr::String(val) => write!(f, "{}", val),
            ValueRepr::Nil() => write!(f, "NIL"),
        }
    }
}

impl Value {
    pub fn new() -> Self {
        Self { 0: Default::default() }
    }

    pub fn print(&self) {
        println!("{}", self)
    }
}
