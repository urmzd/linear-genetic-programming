use core::fmt;
use std::fmt::Display;

use num_derive::{FromPrimitive, ToPrimitive};
use serde::{Deserialize, Serialize};
use strum::EnumCount;

use lgp::{
    core::{
        algorithm::{GeneticAlgorithm, Loader},
        inputs::ValidInput,
        program::Program,
        registers::O32,
    },
    extensions::classification::{ClassificationInput, ClassificationParameters},
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

pub struct IrisLgp;

impl GeneticAlgorithm for IrisLgp {
    type O = Program<ClassificationParameters<IrisInput>>;
}

impl<'a> Loader for IrisLgp {
    type InputType = IrisInput;
}

#[derive(Deserialize, Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Serialize, Hash)]
pub struct IrisInput {
    sepal_length: O32,
    sepal_width: O32,
    petal_length: O32,
    petal_width: O32,
    class: IrisClass,
}

impl ClassificationInput for IrisInput {
    fn get_class(&self) -> usize {
        self.class as usize
    }
}

impl Display for IrisInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let serialized = toml::to_string(&self).unwrap();
        f.write_str(&serialized)
    }
}

impl ValidInput for IrisInput {
    const N_INPUT_REGISTERS: usize = 4;
    const N_ACTION_REGISTERS: usize = 3;

    fn flat(&self) -> Vec<O32> {
        [
            self.sepal_length,
            self.sepal_width,
            self.petal_length,
            self.petal_width,
        ]
        .to_vec()
    }
}
