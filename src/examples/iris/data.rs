use core::fmt;
use std::{fmt::Display, marker::PhantomData};

use num_derive::FromPrimitive;
use serde::{Deserialize, Serialize};
use strum::EnumCount;

use crate::{
    core::{
        algorithm::{GeneticAlgorithm, Loader},
        program::Program,
        registers::{RegisterValue, Registers},
    },
    extensions::classification::{Classification, ClassificationInput},
    utils::common_traits::{Compare, Show, ValidInput},
};

pub const IRIS_DATASET_LINK: &'static str =
    "https://archive.ics.uci.edu/ml/machine-learning-databases/iris/bezdekIris.data";

#[derive(
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    EnumCount,
    PartialOrd,
    Ord,
    strum::Display,
    Serialize,
    Deserialize,
    Hash,
    FromPrimitive,
)]
pub enum IrisClass {
    #[serde(rename = "Iris-setosa")]
    Setosa = 0,
    #[serde(rename = "Iris-versicolor")]
    Versicolour = 1,
    #[serde(rename = "Iris-virginica")]
    Virginica = 2,
}

pub struct IrisLgp<'a>(PhantomData<&'a ()>);

impl<'a> GeneticAlgorithm<'a> for IrisLgp<'a> {
    type O = Program<'a, Classification<'a, IrisInput>>;
}

impl<'a> Loader for IrisLgp<'a> {
    type InputType = IrisInput;
}

#[derive(Deserialize, Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Hash)]
pub struct IrisInput {
    sepal_length: RegisterValue,
    sepal_width: RegisterValue,
    petal_length: RegisterValue,
    petal_width: RegisterValue,
    class: IrisClass,
}

impl ClassificationInput for IrisInput {
    fn get_class(&self) -> Self::Represent {
        self.class
    }
}

impl Compare for IrisClass {}

impl Display for IrisInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let serialized = toml::to_string(&self).unwrap();
        f.write_str(&serialized)
    }
}

impl Show for IrisInput {}
impl Compare for IrisInput {}

impl ValidInput for IrisInput {
    const N_OUTPUTS: usize = 3;
    const N_INPUTS: usize = 4;

    type Represent = IrisClass;
}

impl From<IrisInput> for Registers {
    fn from(input: IrisInput) -> Self {
        Registers::from(vec![
            input.sepal_length,
            input.sepal_width,
            input.petal_length,
            input.petal_width,
        ])
    }
}
