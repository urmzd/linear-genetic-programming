use num::{FromPrimitive, ToPrimitive};
use strum::EnumCount;

use super::registers::{RegisterValue, Registers};

pub type Inputs<InputType> = Vec<InputType>;

pub trait ValidInput: Clone
where
    Self::Actions: Clone + FromPrimitive + EnumCount + ToPrimitive,
    for<'a> Registers: From<&'a Self>,
{
    type Actions;

    const N_INPUTS: usize;

    fn flat(&self) -> Vec<RegisterValue>;
}

impl<T> From<&T> for Registers
where
    T: ValidInput,
{
    fn from(input: &T) -> Self {
        input.flat().into()
    }
}
