use core::fmt;
use std::{fmt::Display, marker::PhantomData};

use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use strum::EnumCount;

use lgp::{
    core::{
        algorithm::{GeneticAlgorithm, Loader},
        characteristics::{Compare, Show},
        inputs::ValidInput,
        instruction::{Mode, Modes},
        program::Program,
        registers::RegisterValue,
    },
    extensions::classification::{ClassificationInput, ClassificationParameters},
    utils::executables::{Executables, DEFAULT_EXECUTABLES},
};

use std::error;

use tempfile::NamedTempFile;

use std::io::Write;

pub struct ContentFilePair(pub String, pub NamedTempFile);

pub async fn get_iris_content() -> Result<ContentFilePair, Box<dyn error::Error>> {
    let tmp_file = NamedTempFile::new()?;
    let response = reqwest::get(IRIS_DATASET_LINK).await?;
    let content = response.text().await?;
    writeln!(&tmp_file, "{}", &content)?;

    Ok(ContentFilePair(content, tmp_file))
}

pub const IRIS_EXECUTABLES: Executables = DEFAULT_EXECUTABLES;

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
    ToPrimitive,
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
    type O = Program<'a, ClassificationParameters<'a, IrisInput>>;
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
    fn get_class(&self) -> Self::Actions {
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
    type Actions = IrisClass;

    const AVAILABLE_EXECUTABLES: Executables = IRIS_EXECUTABLES;
    const AVAILABLE_MODES: Modes = Mode::ALL;

    const N_INPUTS: usize = 4;

    fn argmax(mut ties: Vec<usize>) -> Option<Self::Actions> {
        if ties.len() > 1 {
            return None;
        } else {
            return FromPrimitive::from_usize(ties.pop().unwrap());
        }
    }

    fn as_register_values(&self) -> Vec<RegisterValue> {
        [
            self.sepal_length,
            self.sepal_width,
            self.petal_length,
            self.petal_width,
        ]
        .to_vec()
    }
}
