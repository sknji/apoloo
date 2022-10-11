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
pub(crate) enum ValueRepr {
    Nil(),
    Boolean(bool),
    Number(f64),
    String(String),
    Function(i8, Bytecodes, String),
}

#[derive(Debug, Clone)]
pub struct Value(pub(crate) ValueRepr);

impl Neg for Value {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self.0 {
            ValueRepr::Boolean(v) => Value(ValueRepr::Boolean(!v)),
            ValueRepr::Number(v) => Value(ValueRepr::Number(-v)),
            _ => Value(ValueRepr::Nil())
        }
    }
}

impl Add for Value {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => {
                Value(ValueRepr::Number(l + r))
            }
            (ValueRepr::String(l), ValueRepr::String(r)) => {
                Value(ValueRepr::String(l + &r))
            }
            _ => Value(ValueRepr::Nil())
        }
    }
}

impl Sub for Value {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => {
                Value(ValueRepr::Number(l - r))
            }
            _ => Value(ValueRepr::Nil())
        }
    }
}

impl Mul for Value {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => {
                Value(ValueRepr::Number(l * r))
            }
            (ValueRepr::String(l), ValueRepr::Number(t)) => {
                Value(ValueRepr::String(l.repeat(t as usize)))
            }
            _ => Value(ValueRepr::Nil())
        }
    }
}

impl Div for Value {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        match (self.0, rhs.0) {
            (ValueRepr::Number(l), ValueRepr::Number(r)) => {
                Value(ValueRepr::Number(l / r))
            }
            _ => Value(ValueRepr::Nil())
        }
    }
}

impl ValueRepr {
    pub(crate) fn kind(&self) -> ValueKind {
        match &self {
            ValueRepr::Boolean(_) => ValueKind::Bool,
            ValueRepr::Number(_) => ValueKind::Number,
            ValueRepr::String(_) => ValueKind::String,
            ValueRepr::Function(_, _, _) => ValueKind::Function,
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
            ValueRepr::Function(_, _, name) => write!(f, "<fn {}>", name),
            ValueRepr::Nil() => write!(f, "NIL"),
        }
    }
}

impl Value {
    pub(crate) fn new() -> Self {
        Self { 0: Default::default() }
    }

    pub(crate) fn print(&self) {
        println!("{}", self)
    }
}