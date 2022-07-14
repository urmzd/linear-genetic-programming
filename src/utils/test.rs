// For testing purposes only (binary classification).

use std::marker::PhantomData;

use derive_new::new;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use ordered_float::OrderedFloat;
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use strum::EnumCount;

use crate::{
    core::{
        algorithm::GeneticAlgorithm,
        instruction::{Mode, Modes},
        program::Program,
        registers::Registers,
    },
    extensions::classification::{ClassificationInput, ClassificationParameters},
};

use super::{
    common_traits::{Compare, Executables, Show, ValidInput, DEFAULT_EXECUTABLES},
    random::generator,
};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Debug, Serialize, Deserialize, new)]
pub struct TestInput<'a>(pub [usize; 5], PhantomData<&'a ()>);

impl<'a> Into<Registers<'a>> for TestInput<'a> {
    fn into(self) -> Registers<'a> {
        Registers::new(
            self.0
                .to_vec()
                .iter()
                .map(|v| OrderedFloat(*v as f32))
                .collect(),
            2,
            1,
        )
    }
}
impl<'a> Compare for TestInput<'a> {}
impl<'a> Show for TestInput<'a> {}

#[derive(
    Eq, PartialEq, Ord, PartialOrd, FromPrimitive, Hash, Clone, EnumCount, num_derive::ToPrimitive,
)]
pub enum TestRepresent {
    One = 0,
    Two = 1,
}

impl Compare for TestRepresent {}

impl<'a> ValidInput for TestInput<'a> {
    type Actions = TestRepresent;

    const N_INPUTS: usize = 4;

    fn argmax(ties: Vec<usize>) -> Option<Self::Actions> {
        FromPrimitive::from_usize(*ties.choose(&mut generator()).unwrap())
    }

    const AVAILABLE_EXECUTABLES: Executables = DEFAULT_EXECUTABLES;

    const AVAILABLE_MODES: Modes = Mode::ALL;
}

impl<'a> ClassificationInput for TestInput<'a> {
    fn get_class(&self) -> TestRepresent {
        FromPrimitive::from_usize(self.0[Self::N_INPUTS]).unwrap()
    }
}

pub struct TestLgp<'a>(PhantomData<&'a ()>);
impl<'a> GeneticAlgorithm<'a> for TestLgp<'a> {
    type O = Program<'a, ClassificationParameters<'a, TestInput<'a>>>;
}

pub const DEFAULT_INPUTS: &'static [TestInput] = &[
    TestInput::new([0; 5]),
    TestInput::new([1; 5]),
    TestInput::new([0, 0, 0, 1, 1]),
    TestInput::new([0, 1, 1, 1, 1]),
];
