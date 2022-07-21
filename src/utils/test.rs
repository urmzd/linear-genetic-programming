// For testing purposes only (binary classification).

use derive_new::new;
use num_derive::FromPrimitive;
use ordered_float::OrderedFloat;
use rand::{distributions::Standard, prelude::Distribution};
use serde::{Deserialize, Serialize};
use strum::EnumCount;

use crate::{
    core::{
        algorithm::GeneticAlgorithm, inputs::ValidInput, program::Program, registers::RegisterValue,
    },
    extensions::classification::{ClassificationInput, ClassificationParameters},
};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Debug, Serialize, Deserialize, new)]
pub struct TestInput(pub [OrderedFloat<f32>; 5]);

#[derive(
    Eq, PartialEq, Ord, PartialOrd, FromPrimitive, Hash, Clone, EnumCount, num_derive::ToPrimitive,
)]
pub enum TestRepresent {
    One = 0,
    Two = 1,
}

impl ValidInput for TestInput {
    const N_INPUT_REGISTERS: usize = 4;
    const N_ACTION_REGISTERS: usize = 2;

    fn flat(&self) -> Vec<RegisterValue> {
        vec![self.0[0], self.0[1], self.0[2], self.0[3]]
    }
}

impl ClassificationInput for TestInput {
    fn get_class(&self) -> usize {
        self.0[Self::N_INPUT_REGISTERS].into_inner() as usize
    }
}

pub struct TestLgp;
impl GeneticAlgorithm for TestLgp {
    type O = Program<ClassificationParameters<TestInput>>;
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
