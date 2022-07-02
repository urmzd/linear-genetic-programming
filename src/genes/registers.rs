use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use more_asserts::assert_le;
use ordered_float::OrderedFloat;
use serde::Serialize;

use crate::utils::common_traits::ValidInput;

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

    // TODO: compute ties
    pub fn argmax<V>(&self, break_ties_fn: impl FnOnce(Vec<usize>) -> V) -> V
    where
        V: ValidInput,
    {
        let mut arg_lookup: HashMap<RegisterValue, HashSet<usize>> = HashMap::new();

        let Registers(registers) = &self;

        for index in 0..V::N_OUTPUTS {
            let value = registers.get(index).unwrap();
            if arg_lookup.contains_key(value) {
                arg_lookup.get_mut(value).unwrap().insert(index);
            } else {
                arg_lookup.insert(*registers.get(index).unwrap(), HashSet::from([index]));
            }
        }

        let max_value = arg_lookup.keys().max().unwrap().to_owned();
        let indices = arg_lookup.remove(&max_value).unwrap();

        break_ties_fn(indices.into_iter().collect())
    }
}
