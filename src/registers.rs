use std::collections::{HashMap, HashSet};

use ordered_float::OrderedFloat;
use std::fmt;

pub type RegisterValue = OrderedFloat<f32>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Registers(Vec<RegisterValue>);

impl Registers {
    pub fn new(n_registers: usize) -> Registers {
        Registers(vec![OrderedFloat(0f32); n_registers])
    }

    pub fn from(vec: Vec<RegisterValue>) -> Registers {
        Registers(vec)
    }

    pub fn reset(&mut self) -> () {
        let Registers(internal_registers) = self;

        for index in 0..internal_registers.len() {
            internal_registers[index] = OrderedFloat(0f32);
        }
    }

    pub fn len(&self) -> usize {
        let Registers(internal_registers) = &self;
        internal_registers.len()
    }

    pub fn update(&mut self, index: usize, value: RegisterValue) -> () {
        let Registers(internal_values) = self;
        internal_values[index] = value
    }

    pub fn get_value_at_index(&self, index: usize) -> RegisterValue {
        let len = self.len();
        if index < len {
            return self.0[index];
        }

        panic!("Invalid Index: {}; Len: {}", index, len)
    }

    /// Returns:
    ///  `desired_index` if argmax is desired_index else None.
    pub fn argmax(&self, n_classes: usize, desired_index: usize) -> Option<usize> {
        let mut arg_lookup: HashMap<OrderedFloat<f32>, HashSet<usize>> = HashMap::new();

        let Registers(registers) = &self;

        for index in 0..n_classes {
            let value = registers.get(index).unwrap();
            if arg_lookup.contains_key(value) {
                arg_lookup.get_mut(value).unwrap().insert(index);
            } else {
                arg_lookup.insert(*registers.get(index).unwrap(), HashSet::from([index]));
            }
        }

        let max_value = arg_lookup.keys().max().unwrap();
        let indices = arg_lookup.get(max_value).unwrap();

        if indices.contains(&desired_index) {
            if indices.len() == 1 {
                return Some(desired_index);
            }
        }

        None
    }
}

pub trait RegisterRepresentable:
    fmt::Debug + Into<Registers> + Clone + PartialEq + Eq + PartialOrd + Ord
{
    fn get_number_classes() -> usize;
    fn get_number_features() -> usize;
}
