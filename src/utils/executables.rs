use ordered_float::OrderedFloat;

use crate::core::registers::RegisterValue;

pub type Op = fn(a: RegisterValue, b: RegisterValue) -> RegisterValue;

pub type Executables = &'static [Op];

pub const DEFAULT_EXECUTABLES: Executables = &[add, subtract, multiply, divide];

pub fn add(a: RegisterValue, b: RegisterValue) -> RegisterValue {
    a + b
}

pub fn subtract(a: RegisterValue, b: RegisterValue) -> RegisterValue {
    a - b
}

pub fn multiply(a: RegisterValue, b: RegisterValue) -> RegisterValue {
    a * b
}

pub fn divide(a: RegisterValue, _b: RegisterValue) -> RegisterValue {
    a / OrderedFloat(2f32)
}
