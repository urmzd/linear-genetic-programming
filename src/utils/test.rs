// For testing purposes only (binary classification).

use std::marker::PhantomData;

use derive_new::new;
use num_derive::FromPrimitive;
use ordered_float::OrderedFloat;
use rand::{distributions::Standard, prelude::Distribution};
use serde::{Deserialize, Serialize};
use strum::EnumCount;

use crate::{
    core::{
        algorithm::GeneticAlgorithm,
        characteristics::{Compare, Show},
        inputs::ValidInput,
        program::Program,
        registers::RegisterValue,
    },
    extensions::classification::ClassificationInput,
};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Debug, Serialize, Deserialize, new)]
pub struct TestInput(pub [OrderedFloat<f32>; 5]);

impl Compare for TestInput {}
impl Show for TestInput {}

#[derive(
    Eq, PartialEq, Ord, PartialOrd, FromPrimitive, Hash, Clone, EnumCount, num_derive::ToPrimitive,
)]
pub enum TestRepresent {
    One = 0,
    Two = 1,
}

impl Compare for TestRepresent {}

impl ValidInput for TestInput {
    type Actions = TestRepresent;

    const N_INPUTS: usize = 4;

    fn flat(&self) -> Vec<RegisterValue> {
        vec![self.0[0], self.0[1], self.0[2], self.0[3]]
    }

    fn argmax(registers: &crate::core::registers::Registers) -> i32 {
        todo!()
    }
}

impl ClassificationInput for TestInput {
    fn get_class(&self) -> usize {
        self.0[Self::N_INPUTS].into_inner() as usize
    }
}

pub struct TestLgp<'a>(PhantomData<&'a ()>);
impl<'a> GeneticAlgorithm<'a> for TestLgp<'a> {
    type O = Program<TestInput>;
}

impl Default for TestInput {
    fn default() -> Self {
        TestInput::new([0; 5].map(|v| OrderedFloat(v as f32)))
    }
}

impl Distribution<TestInput> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> TestInput {
        let data = [0; 5].map(|_| OrderedFloat(rng.gen_range::<usize, _>(0..=1) as f32));
        TestInput(data)
    }
}
