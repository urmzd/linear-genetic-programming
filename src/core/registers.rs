use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

use strum::EnumCount;

use derive_new::new;
use more_asserts::assert_le;
use ordered_float::OrderedFloat;
use serde::Serialize;

use super::inputs::ValidInput;

pub type RegisterValue = OrderedFloat<f32>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, new)]
pub struct Registers<'a> {
    data: Vec<MaybeBorrowed<'a, RegisterValue>>,
    n_outputs: usize,
    n_extras: usize,
    frozen: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, new, Hash)]
pub enum MaybeBorrowed<'a, T>
where
    T: Clone,
{
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> MaybeBorrowed<'a, T>
where
    T: Clone,
{
    pub fn is_owned(&self) -> bool {
        match self {
            MaybeBorrowed::Borrowed(_) => false,
            MaybeBorrowed::Owned(_) => true,
        }
    }

    pub fn get_borrowed(&self) -> Option<&'a T> {
        match self {
            MaybeBorrowed::Borrowed(value) => Some(value),
            _ => None,
        }
    }

    pub fn get_owned(&self) -> Option<T> {
        match self {
            MaybeBorrowed::Owned(value) => Some(value.clone()),
            _ => None,
        }
    }

    pub fn get(&self) -> T {
        match self {
            MaybeBorrowed::Owned(value) => value,
            MaybeBorrowed::Borrowed(value) => value,
        }
        .clone()
    }
}

#[derive(Debug, Clone, new, Serialize)]
pub struct RegisterGeneratorParameters {
    n_extra_action_registers: usize,
}

impl<'a> Registers<'a> {
    pub fn generate<T: ValidInput>(parameters: &RegisterGeneratorParameters) -> Self {
        let RegisterGeneratorParameters {
            n_extra_action_registers,
        } = parameters;

        Registers::new(
            vec![
                MaybeBorrowed::Owned(OrderedFloat(0.));
                T::Actions::COUNT + *n_extra_action_registers
            ],
            *n_extra_action_registers,
            T::Actions::COUNT,
            false,
        )
    }

    pub fn reset(&mut self) -> &mut Self {
        if !self.frozen {
            let Registers { data, .. } = self;
            for value in data.as_mut_slice() {
                if value.is_owned() {
                    *value = MaybeBorrowed::Owned(OrderedFloat(0f32))
                }
            }
        }
        self
    }

    pub fn len(&self) -> usize {
        self.n_extras + self.n_outputs
    }

    pub fn update(&mut self, index: usize, value: RegisterValue) {
        if !self.frozen {
            let Registers { data, .. } = self;
            data[index] = MaybeBorrowed::Owned(value);
        }
    }

    pub fn get(&self, index: usize) -> &MaybeBorrowed<RegisterValue> {
        self.data.get(index).unwrap()
    }

    pub fn as_mut_slice<'b>(
        &'b mut self,
        start: usize,
        end: Option<usize>,
    ) -> &'b mut [MaybeBorrowed<'a, RegisterValue>] {
        let range = Range {
            start,
            end: end.unwrap_or(start + 1),
        };

        assert_le!(range.end, self.data.len());

        &mut self.data[range]
    }

    pub fn as_slice(
        &'a self,
        start: usize,
        end: Option<usize>,
    ) -> &'a [MaybeBorrowed<RegisterValue>] {
        let range = Range {
            start,
            end: end.unwrap_or(start + 1),
        };

        assert_le!(range.end, self.data.len());

        &self.data[range]
    }

    pub fn argmax(&self) -> Vec<usize> {
        let mut arg_lookup: HashMap<&MaybeBorrowed<'a, RegisterValue>, HashSet<usize>> =
            HashMap::new();

        let Registers {
            data, n_outputs, ..
        } = &self;

        for index in 0..(*n_outputs) {
            let value = data.get(index).unwrap();
            if arg_lookup.contains_key(value) {
                arg_lookup.get_mut(value).unwrap().insert(index);
            } else {
                arg_lookup.insert(data.get(index).unwrap(), HashSet::from([index]));
            }
        }

        let max_value = arg_lookup.keys().max().unwrap().to_owned();
        let indices = arg_lookup.remove(&max_value).unwrap();
        let indices_vec = indices.into_iter().collect();

        indices_vec
    }
}
