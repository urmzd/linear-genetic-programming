use derive_new::new;
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

impl Modes {
    pub fn new(include_external: bool, include_internal: bool) -> Vec<Modes> {
        let mut modes_available = vec![];
        if include_internal {
            modes_available.push(Modes::Internal)
        }

        if include_external {
            modes_available.push(Modes::External)
        }

        modes_available
    }

    pub fn all() -> Vec<Modes> {
        Self::new(true, true)
    }
}

#[derive(Clone, Serialize)]
pub struct Instruction<'a> {
    source_index: usize,
    target_index: usize,
    mode: Modes,
    #[serde(skip_serializing)]
    executable: AnyExecutable,
    parameters_used: &'a InstructionGeneratorParameters,
}

impl<'a> Generate<'a> for Instruction<'a> {
    type GeneratorParameters = InstructionGeneratorParameters;

    fn generate(parameters: &'a Self::GeneratorParameters) -> Self {
        let InstructionGeneratorParameters {
            n_registers,
            n_inputs,
            modes_available,
            executables_available: executables,
        } = parameters;

        let source_index = UniformInt::<usize>::new(0, n_registers).sample(&mut generator());

        let mode = modes_available.choose(&mut generator()).unwrap().clone();

        let upper_bound_target_index = if mode == Modes::External {
            n_inputs.unwrap()
        } else {
            *n_registers
        };
        let target_index =
            UniformInt::<usize>::new(0, upper_bound_target_index).sample(&mut thread_rng());

        let exec = executables.choose(&mut generator()).unwrap().to_owned();

        Instruction {
            source_index,
            target_index,
            executable: exec,
            mode,
            parameters_used: &parameters,
        }
    }
}

impl<'b> Show for InstructionGeneratorParameters {}
#[derive(Clone, Debug, Serialize, new)]
pub struct InstructionGeneratorParameters {
    pub n_registers: usize,
    pub n_inputs: Option<usize>,
    pub modes_available: Vec<Modes>,
    #[serde(skip_serializing)]
    pub executables_available: Executables,
}

impl<'b> Eq for Instruction<'b> {}

impl<'b> PartialEq for Instruction<'b> {
    fn eq(&self, other: &Self) -> bool {
        self.source_index == other.source_index
            && self.target_index == other.target_index
            && self.mode == other.mode
            && self.executable.get_fn() as usize == other.executable.get_fn() as usize
    }
}

impl<'b> Debug for Instruction<'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("mode", &self.mode)
            .field("source_index", &self.source_index)
            .field("target_index", &self.target_index)
            .finish()
    }
}

impl<'b> Mutate for Instruction<'b> {
    fn mutate(&self) -> Self {
        let mut mutated = Self::generate(&self.parameters_used);

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
            mutated.executable = self.executable.clone();
        }

        mutated
    }
}

impl<'b> Show for Instruction<'b> {}

impl<'b> Instruction<'b> {
    fn get_data<InputType>(&self, registers: &Registers, data: &InputType) -> Registers
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
        (self.executable.get_fn())(source_slice, target_slice);
    }
}
