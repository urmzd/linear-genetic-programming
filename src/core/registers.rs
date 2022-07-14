use std::collections::{HashMap, HashSet};

use strum::EnumCount;

use derive_new::new;
use ordered_float::OrderedFloat;
use serde::Serialize;

use super::inputs::ValidInput;

pub type RegisterValue = OrderedFloat<f32>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, new)]
pub struct Registers {
    data: Vec<RegisterValue>,
    n_outputs: usize,
    n_extras: usize,
}

#[derive(Debug, Clone, new, Serialize)]
pub struct RegisterGeneratorParameters {
    n_extras: usize,
}

impl Registers {
    pub fn generate<T: ValidInput>(parameters: &RegisterGeneratorParameters) -> Self {
        let RegisterGeneratorParameters { n_extras } = parameters.clone();

        let n_outputs = T::Actions::COUNT;
        let data = vec![OrderedFloat(0.); n_outputs + n_extras];

        Registers::new(data, n_outputs, n_extras)
    }

    pub fn reset(&mut self) {
        let Registers { data, .. } = self;
        for value in data.as_mut_slice() {
            *value = OrderedFloat(0.)
        }
    }

    pub fn reset_clone(&self) -> Self {
        let mut registers = self.clone();
        registers.reset();
        registers
    }

    pub fn len(&self) -> usize {
        self.n_extras + self.n_outputs
    }

    pub fn update(&mut self, index: usize, value: RegisterValue) {
        let Registers { data, .. } = self;
        data[index] = value;
    }

    pub fn get(&self, index: usize) -> &RegisterValue {
        self.data.get(index).unwrap()
    }

    pub fn get_owned(&self, index: usize) -> RegisterValue {
        self.data.get(index).map(|v| v.clone()).unwrap()
    }

    pub fn argmax(&self) -> Vec<usize> {
        let mut arg_lookup: HashMap<&RegisterValue, HashSet<usize>> = HashMap::new();

        let Registers {
            data, n_outputs, ..
        } = &self;

        for index in 0..(*n_outputs) {
            let value = data.get(index).unwrap();
            if arg_lookup.contains_key(value) {
                arg_lookup.get_mut(value).unwrap().insert(index);
            } else {
                let new_set = HashSet::from([index]);
                arg_lookup.insert(value, new_set);
            }
        }

        let max_value = arg_lookup.keys().max().unwrap().to_owned();
        let indices = arg_lookup.remove(&max_value).unwrap();
        let indices_vec = indices.into_iter().collect();

        indices_vec
    }
}
