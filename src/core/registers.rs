use std::{ops::Index, slice::SliceIndex};

use serde::Serialize;

pub type R32 = f32;

#[derive(Debug, Clone, Serialize)]
pub struct Registers(Vec<R32>);

impl From<Vec<R32>> for Registers {
    fn from(data: Vec<R32>) -> Self {
        Registers(data)
    }
}

impl Registers {
    pub fn new(n_registers: usize) -> Self {
        let data = vec![0.; n_registers];

        Registers(data)
    }

    pub fn reset(&mut self) {
        let Registers(data) = self;
        for value in data.as_mut_slice() {
            *value = 0.
        }
    }

    pub fn duplicate(&self) -> Self {
        Self::new(self.len())
    }

    pub fn len(&self) -> usize {
        let Registers(data) = self;
        data.len()
    }

    pub fn update(&mut self, index: usize, value: R32) {
        let Registers(data) = self;
        data[index] = value;
    }

    pub fn get(&self, index: usize) -> &R32 {
        let Registers(data) = self;
        data.get(index).unwrap()
    }
}

impl<Idx> Index<Idx> for Registers
where
    Idx: SliceIndex<[R32]>,
{
    type Output = Idx::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}
