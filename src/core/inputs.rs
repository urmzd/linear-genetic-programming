use super::registers::{Registers, R32};

pub type Inputs<InputType> = Vec<InputType>;

pub trait ValidInput: Clone
where
    for<'a> Registers: From<&'a Self>,
{
    const N_INPUT_REGISTERS: usize;
    const N_ACTION_REGISTERS: usize;

    fn flat(&self) -> Vec<R32>;
}

impl<T> From<&T> for Registers
where
    T: ValidInput,
{
    fn from(input: &T) -> Self {
        input.flat().into()
    }
}
