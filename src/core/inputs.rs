use num::{FromPrimitive, ToPrimitive};
use rand::prelude::SliceRandom;
use strum::EnumCount;

use crate::utils::{executables::Executables, random::generator};

use super::{
    characteristics::Show,
    instruction::Modes,
    registers::{MaybeBorrowed, RegisterValue, Registers},
};

pub type Inputs<InputType> = Vec<InputType>;

pub trait ValidInput: Show + Clone
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

    fn as_register_values(&self) -> Vec<RegisterValue>;
}

impl<'a, T: ValidInput> From<&'a T> for Registers<'a> {
    fn from(input: &'a T) -> Self {
        let ref_data: Vec<MaybeBorrowed<RegisterValue>> = input
            .as_register_values()
            .iter()
            .map(|v| MaybeBorrowed::Owned(*v))
            .collect();
        Registers::new(ref_data, 2, 0, true)
    }
}
