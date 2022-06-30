use core::fmt;

use num::FromPrimitive;
use serde::{de::DeserializeOwned, Serialize};

use crate::genes::registers::{RegisterValue, Registers};

#[derive(Clone)]
pub struct AnyExecutable(pub &'static str, pub InternalFn);

impl AnyExecutable {
    pub fn get_name(&self) -> &'static str {
        &self.0
    }

    pub fn get_fn(&self) -> InternalFn {
        self.1
    }
}

type InternalFn =
    for<'r, 's> fn(&'r mut [RegisterValue], &'s [RegisterValue]) -> &'r [RegisterValue];

impl fmt::Debug for AnyExecutable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AnyExecutable").field(&self.0).finish()
    }
}

pub type Executables = &'static [AnyExecutable];

pub trait Compare<V = Self>: PartialEq<V> + Eq + PartialOrd + Ord {}
pub trait Show: fmt::Debug + Serialize {}

pub type Inputs<InputType> = Vec<InputType>;

pub trait ValidInput: Clone + Compare + Show + DeserializeOwned + Into<Registers>
where
    Self::Represent: Compare + FromPrimitive,
{
    const N_CLASSES: usize;
    const N_FEATURES: usize;

    type Represent;

    fn argmax(&self, registers: &Registers) -> Vec<Self::Represent>;
}
