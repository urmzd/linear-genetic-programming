use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::Range,
};

use derive_new::new;
use more_asserts::assert_le;
use ordered_float::OrderedFloat;
use serde::Serialize;
use strum::EnumCount;

use crate::utils::common_traits::ValidInput;

pub type RegisterValue = OrderedFloat<f32>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, new)]
pub struct Registers {
    data: Vec<RegisterValue>,
    n_outputs: usize,
    n_extras: usize,
}

#[derive(Clone, Debug, Serialize, new)]
pub struct RegisterGeneratorParameters<T>
where
    T: ValidInput,
{
    n_extras: usize,
    marker: PhantomData<T>,
}

impl Registers {
    pub fn generate<'a, T>(parameters: &'a RegisterGeneratorParameters<T>) -> Self
    where
        T: ValidInput,
    {
        let RegisterGeneratorParameters { n_extras, .. } = parameters;

        let n_outputs = T::Actions::COUNT;

        let data = (0..(n_outputs + n_extras))
            .map(T::generate_register_value_from)
            .collect();

        Registers {
            data,
            n_outputs,
            n_extras: *n_extras,
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
