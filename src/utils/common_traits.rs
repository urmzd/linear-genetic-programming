use core::fmt;

use num::{FromPrimitive, ToPrimitive};
use ordered_float::OrderedFloat;
use rand::prelude::SliceRandom;
use serde::Serialize;
use strum::EnumCount;

use crate::{
    core::{
        instruction::Modes,
        registers::{RegisterValue, Registers},
    },
    executable, executables,
};

use super::random::generator;

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

executable!(add, +);
executable!(subtract, -);
executable!(divide, /, OrderedFloat(2f32));
executable!(multiply, *);

pub const DEFAULT_EXECUTABLES: Executables = executables!(add, subtract, divide, multiply);
pub trait Compare<V = Self>: PartialEq<V> + Eq + PartialOrd<V> + Ord {}
pub trait Show: fmt::Debug + Serialize {}

pub type Inputs<InputType> = Vec<InputType>;

pub trait ValidInput: Show
where
    Self::Actions: Clone + FromPrimitive + EnumCount + ToPrimitive,
    for<'a> Registers<'a>: From<&'a Self>,
{
    type Actions;

    const N_INPUTS: usize;

    const AVAILABLE_EXECUTABLES: Executables;
    const AVAILABLE_MODES: Modes;

    fn argmax(ties: Vec<usize>) -> Option<Self::Actions> {
        FromPrimitive::from_usize(*ties.choose(&mut generator()).unwrap())
    }
}
