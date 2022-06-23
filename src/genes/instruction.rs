use num::FromPrimitive;
use num_derive::FromPrimitive;
use rand::distributions::uniform::{UniformInt, UniformSampler};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use strum::EnumCount;

use crate::utils::common_traits::{AnyExecutable, Executables, Show, ValidInput};
use crate::utils::random::generator;

use super::characteristics::{Generate, Mutate};
use super::registers::Registers;

#[derive(FromPrimitive, Clone, Debug, EnumCount, PartialEq, Eq, Serialize)]
pub enum Modes {
    External = 0,
    Internal = 1,
}

#[derive(Clone, Serialize)]
pub struct Instruction<'a> {
    source_index: usize,
    target_index: usize,
    mode: Modes,
    #[serde(skip_serializing)]
    exec: AnyExecutable,
    params_used: &'a InstructionGenerateParams,
}

impl<'a> Generate<'a> for Instruction<'a> {
    type GenerateParamsType = InstructionGenerateParams;

    fn generate(parameters: &'a Self::GenerateParamsType) -> Self {
        let InstructionGenerateParams {
            n_features: data_len,
            n_registers: registers_len,
            executables,
        } = parameters;

        let source_index = UniformInt::<usize>::new(0, registers_len).sample(&mut generator());

        let mode = FromPrimitive::from_usize(generator().gen_range(0..=1)).unwrap();

        let target_index = UniformInt::<usize>::new(
            0,
            if mode == Modes::External {
                data_len
            } else {
                registers_len
            },
        )
        .sample(&mut thread_rng());

        let exec = executables.choose(&mut generator()).unwrap().to_owned();

        Instruction {
            source_index,
            target_index,
            exec,
            mode,
            params_used: &parameters,
        }
    }
}

#[derive(Clone, Debug, Serialize, Copy)]
pub struct InstructionGenerateParams {
    n_registers: usize,
    n_features: usize,
    #[serde(skip_serializing)]
    executables: Executables,
}

impl InstructionGenerateParams {
    pub fn new(registers_len: usize, data_len: usize, executables: Executables) -> Self {
        InstructionGenerateParams {
            n_registers: registers_len,
            n_features: data_len,
            executables,
        }
    }
}

impl<'a> Eq for Instruction<'a> {}

impl<'a> PartialEq for Instruction<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.source_index == other.source_index
            && self.target_index == other.target_index
            && self.mode == other.mode
            && self.exec.get_fn() as usize == other.exec.get_fn() as usize
    }
}

impl<'a> Debug for Instruction<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("mode", &self.mode)
            .field("source_index", &self.source_index)
            .field("target_index", &self.target_index)
            .finish()
    }
}

impl<'a> Mutate for Instruction<'a> {
    fn mutate(&self) -> Self {
        let mut mutated = Self::generate(&self.params_used);

        let swap_target = generator().gen_bool(0.5);
        let swap_source = generator().gen_bool(0.5);
        let swap_exec = generator().gen_bool(0.5);

        // Flip a Coin: Target
        if swap_target {
            mutated.mode = self.mode.clone();
            mutated.target_index = self.target_index;
        }

        // Flip a Coin: Source
        if swap_source {
            mutated.source_index = self.source_index;
        }

        // Flip a Coin: Executable
        if swap_exec {
            mutated.exec = self.exec.clone();
        }

        mutated
    }
}

impl<'a> Show for Instruction<'a> {}
impl Show for InstructionGenerateParams {}

impl<'a> Instruction<'a> {
    fn get_data<InputType>(&self, registers: &'a Registers, data: &'a InputType) -> Registers
    where
        InputType: ValidInput,
    {
        let target_data: Registers = match self.mode {
            Modes::Internal => registers.clone(),
            Modes::External => data.clone().into(),
        };

        target_data
    }

    pub fn apply<T>(&self, registers: &mut Registers, input: &T)
    where
        T: ValidInput,
    {
        let data = self.get_data(registers, input);
        let target_slice = data.get_slice(self.target_index, None);
        let source_slice = registers.get_mut_slice(self.source_index, None);
        (self.exec.get_fn())(source_slice, target_slice);
    }
}
