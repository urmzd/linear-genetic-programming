// For testing purposes only (binary classification).

use serde::{Deserialize, Serialize};

use crate::genes::registers::Registers;

use super::common_traits::{Compare, Show, ValidInput};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct TestInput(pub [usize; 5]);

impl Into<Registers> for TestInput {
    fn into(self) -> Registers {
        todo!()
    }
}
impl Compare for TestInput {}
impl Show for TestInput {}

impl ValidInput for TestInput {
    // 0 or 1
    const N_CLASSES: usize = 2;
    const N_FEATURES: usize = 4;

    fn get_class(&self) -> usize {
        self.0[Self::N_FEATURES]
    }
}
