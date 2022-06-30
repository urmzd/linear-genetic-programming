// For testing purposes only (binary classification).

use num::FromPrimitive;
use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};

use crate::genes::registers::Registers;

use super::{
    common_traits::{Compare, Show, ValidInput},
    problem_types::ClassificationProblem,
};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct TestInput(pub [usize; 5]);

impl Into<Registers> for TestInput {
    fn into(self) -> Registers {
        todo!()
    }
}
impl Compare for TestInput {}
impl Show for TestInput {}

#[derive(Eq, PartialEq, Ord, PartialOrd, FromPrimitive, Hash, Clone)]
pub enum TestRepresent {
    One = 0,
    Two = 1,
}

impl Compare for TestRepresent {}

impl ValidInput for TestInput {
    // 0 or 1
    const N_OUTPUTS: usize = 2;
    const N_INPUTS: usize = 4;

    type Represent = TestRepresent;

    fn argmax(&self, registers: &Registers) -> Option<Self::Represent> {
        let index = registers
            .argmax::<Self>()
            .into_iter()
            .enumerate()
            .max_by_key(|&(_, value)| value)
            .map(|(idx, _)| idx);

        index.and_then(|v| FromPrimitive::from_usize(v))
    }
}

impl ClassificationProblem for TestInput {
    fn get_class(&self) -> TestRepresent {
        FromPrimitive::from_usize(self.0[Self::N_INPUTS]).unwrap()
    }
}
