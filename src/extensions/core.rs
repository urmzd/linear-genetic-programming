use crate::core::registers::Registers;

pub trait ExtensionParameters {
    fn argmax(registers: &Registers) -> i32;
}
