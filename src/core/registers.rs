use std::{ops::Index, slice::SliceIndex};

use ordered_float::OrderedFloat;
use serde::Serialize;

pub type RegisterValue = OrderedFloat<f32>;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct Registers(Vec<RegisterValue>);

impl From<Vec<RegisterValue>> for Registers {
    fn from(data: Vec<RegisterValue>) -> Self {
        Registers(data)
    }
}

impl Registers {
    pub fn new(n_registers: usize) -> Self {
        let data = vec![OrderedFloat(0.); n_registers];

        Registers(data)
    }

    pub fn reset(&mut self) {
        let Registers(data) = self;
        for value in data.as_mut_slice() {
            *value = OrderedFloat(0.)
        }
    }

    pub fn duplicate(&self) -> Self {
        Self::new(self.len())
    }

    pub fn len(&self) -> usize {
        let Registers(data) = self;
        data.len()
    }

    pub fn update(&mut self, index: usize, value: RegisterValue) {
        let Registers(data) = self;
        data[index] = value;
    }

    pub fn get(&self, index: usize) -> &RegisterValue {
        let Registers(data) = self;
        data.get(index).unwrap()
    }
}

impl<Idx> Index<Idx> for Registers
where
    Idx: SliceIndex<[RegisterValue]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}
