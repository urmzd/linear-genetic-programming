use crate::core::registers::R32;

pub type Op = fn(a: R32, b: R32) -> R32;

pub type Executables = &'static [Op];

pub const DEFAULT_EXECUTABLES: Executables = &[add, subtract, multiply, divide];

pub fn add(a: R32, b: R32) -> R32 {
    a + b
}

pub fn subtract(a: R32, b: R32) -> R32 {
    a - b
}

pub fn multiply(a: R32, b: R32) -> R32 {
    a * b
}

pub fn divide(a: R32, _b: R32) -> R32 {
    a / 2f32
}
