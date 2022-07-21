use ordered_float::OrderedFloat;

use crate::core::registers::O32;

pub type Op = fn(a: O32, b: O32) -> O32;

pub type Executables = &'static [Op];

pub const DEFAULT_EXECUTABLES: Executables = &[add, subtract, multiply, divide];

pub fn add(a: O32, b: O32) -> O32 {
    a + b
}

pub fn subtract(a: O32, b: O32) -> O32 {
    a - b
}

pub fn multiply(a: O32, b: O32) -> O32 {
    a * b
}

pub fn divide(a: O32, _b: O32) -> O32 {
    a / OrderedFloat(2f32)
}
