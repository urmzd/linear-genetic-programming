use num::{FromPrimitive, ToPrimitive};
use rand::prelude::SliceRandom;
use strum::EnumCount;

use crate::utils::{executables::Executables, random::generator};

use super::{
    characteristics::Show,
    registers::{RegisterValue, Registers},
};

pub type Inputs<InputType> = Vec<InputType>;

pub trait ValidInput: Show + Clone
where
    Self::Actions: Clone + FromPrimitive + EnumCount + ToPrimitive,
    for<'a> Registers: From<&'a Self>,
{
    type Actions;

    const N_INPUTS: usize;

    const AVAILABLE_EXECUTABLES: Executables;

    fn map_register_to_action(ties: Vec<usize>) -> Option<Self::Actions> {
        FromPrimitive::from_usize(*ties.choose(&mut generator()).unwrap())
    }

    fn as_register_values(&self) -> Vec<RegisterValue>;
}

impl<'a, T: ValidInput> From<&'a T> for Registers {
    fn from(input: &'a T) -> Self {
        let ref_data: Vec<RegisterValue> = input.as_register_values().iter().map(|v| *v).collect();
        Registers::new(ref_data, 2, 0)
    }
}
