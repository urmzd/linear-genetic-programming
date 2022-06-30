use std::ops::Range;

use more_asserts::assert_le;
use ordered_float::OrderedFloat;
use serde::Serialize;

pub type RegisterValue = OrderedFloat<f32>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Registers(pub Vec<RegisterValue>);

impl Registers {
    pub fn new(n_registers: usize) -> Registers {
        let internal_vec = vec![OrderedFloat(0f32); n_registers];
        Registers(internal_vec)
    }

    pub fn from(vec: Vec<RegisterValue>) -> Registers {
        Registers(vec)
    }

    pub fn reset(&mut self) -> &mut Self {
        let Registers(internal_registers) = self;

        for index in 0..internal_registers.len() {
            internal_registers[index] = OrderedFloat(0f32);
        }

        self
    }

    pub fn len(&self) -> usize {
        let Registers(internal_registers) = &self;
        internal_registers.len()
    }

    pub fn update(&mut self, index: usize, value: RegisterValue) -> () {
        let Registers(internal_values) = self;
        internal_values[index] = value
    }

    pub fn get(&self, index: usize) -> RegisterValue {
        self.0[index]
    }

    pub fn get_mut_slice(&mut self, start: usize, end: Option<usize>) -> &mut [RegisterValue] {
        let range = Range {
            start,
            end: end.unwrap_or(start + 1),
        };

        assert_le!(range.end, self.0.len());

        &mut self.0[range]
    }

    pub fn get_slice(&self, start: usize, end: Option<usize>) -> &[RegisterValue] {
        let range = Range {
            start,
            end: end.unwrap_or(start + 1),
        };

        assert_le!(range.end, self.0.len());

        &self.0[range]
    }
}
