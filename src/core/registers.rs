use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use strum::EnumCount;

use derive_new::new;
use more_asserts::assert_le;
use ordered_float::OrderedFloat;
use serde::Serialize;

use crate::utils::common_traits::ValidInput;

pub type RegisterValue = OrderedFloat<f32>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, new)]
pub struct Registers {
    data: Vec<RegisterValue>,
    n_outputs: usize,
    n_extras: usize,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct RegisterGeneratorParameters {
    n_extra_action_registers: usize,
}

impl Registers {
    pub fn generate<T: ValidInput>(parameters: &RegisterGeneratorParameters) -> Self {
        let RegisterGeneratorParameters {
            n_extra_action_registers,
        } = parameters;

        Registers {
            data: vec![OrderedFloat(0.); T::Actions::COUNT + *n_extra_action_registers],
            n_extras: *n_extra_action_registers,
            n_outputs: T::Actions::COUNT,
        }
    }

    pub fn reset(&mut self) -> &mut Self {
        let Registers { data, .. } = self;

        for index in 0..data.len() {
            data[index] = OrderedFloat(0f32);
        }

        self
    }

    pub fn len(&self) -> usize {
        self.n_extras + self.n_outputs
    }

    pub fn update(&mut self, index: usize, value: RegisterValue) -> () {
        let Registers { data, .. } = self;
        data[index] = value
    }

    pub fn get(&self, index: usize) -> RegisterValue {
        self.data[index]
    }

    pub fn get_mut_slice(&mut self, start: usize, end: Option<usize>) -> &mut [RegisterValue] {
        let range = Range {
            start,
            end: end.unwrap_or(start + 1),
        };

        assert_le!(range.end, self.data.len());

        &mut self.data[range]
    }

    pub fn get_slice(&self, start: usize, end: Option<usize>) -> &[RegisterValue] {
        let range = Range {
            start,
            end: end.unwrap_or(start + 1),
        };

        assert_le!(range.end, self.data.len());

        &self.data[range]
    }

    pub fn argmax(&self) -> Vec<usize> {
        let mut arg_lookup: HashMap<RegisterValue, HashSet<usize>> = HashMap::new();

        let Registers {
            data, n_outputs, ..
        } = &self;

        for index in 0..(*n_outputs) {
            let value = data.get(index).unwrap();
            if arg_lookup.contains_key(value) {
                arg_lookup.get_mut(value).unwrap().insert(index);
            } else {
                arg_lookup.insert(*data.get(index).unwrap(), HashSet::from([index]));
            }
        }

        let max_value = arg_lookup.keys().max().unwrap().to_owned();
        let indices = arg_lookup.remove(&max_value).unwrap();
        let indices_vec = indices.into_iter().collect();

        indices_vec
    }
}
