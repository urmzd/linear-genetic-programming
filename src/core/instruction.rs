use derive_new::new;
use num::FromPrimitive;
use num_derive::FromPrimitive;
use rand::distributions::uniform::{UniformInt, UniformSampler};
use rand::prelude::SliceRandom;
use rand::Rng;
use serde::Serialize;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use strum::EnumCount;

use crate::utils::executables::Op;
use crate::utils::random::generator;

use super::characteristics::{Generate, Mutate, Show};
use super::inputs::ValidInput;
use super::registers::Registers;

#[derive(FromPrimitive, Clone, Debug, EnumCount, PartialEq, Eq, Serialize)]
pub enum Mode {
    External = 0,
    Internal = 1,
}

impl Show for InstructionGeneratorParameters {}
#[derive(Clone, Debug, Serialize, new)]
pub struct InstructionGeneratorParameters {
    pub n_registers: usize,
    pub n_features: usize,
}

impl InstructionGeneratorParameters {
    pub fn from<T: ValidInput>(n_extras: usize) -> Self {
        InstructionGeneratorParameters::new(
            <T as ValidInput>::Actions::COUNT + n_extras,
            <T as ValidInput>::N_INPUTS,
        )
    }
}

#[derive(Serialize)]
pub struct Instruction {
    source_index: usize,
    target_index: usize,
    mode: Mode,
    #[serde(skip_serializing)]
    executable: Op,
}

impl Clone for Instruction {
    fn clone(&self) -> Self {
        Self {
            source_index: self.source_index.clone(),
            target_index: self.target_index.clone(),
            mode: self.mode.clone(),
            executable: self.executable.clone(),
        }
    }
}

impl Generate for Instruction {
    type GeneratorParameters = InstructionGeneratorParameters;

    fn generate<'a>(parameters: &'a Self::GeneratorParameters) -> Self {
        let InstructionGeneratorParameters {
            n_features: n_inputs,
            n_registers,
            ..
        } = parameters;

        let current_generator = &mut generator();

        let source_index = UniformInt::<usize>::new(0, n_registers).sample(current_generator);

        let mode = FromPrimitive::from_usize(
            UniformInt::<usize>::new_inclusive(0, 1).sample(current_generator),
        )
        .unwrap();

        let upper_bound_target_index = *(if mode == Mode::External {
            n_inputs
        } else {
            n_registers
        });
        let target_index =
            UniformInt::<usize>::new(0, upper_bound_target_index).sample(current_generator);

        let exec = T::AVAILABLE_EXECUTABLES
            .choose(current_generator)
            .unwrap()
            .to_owned();

        Instruction {
            source_index,
            target_index,
            executable: exec,
            mode,
        }
    }
}

impl Eq for Instruction {}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        self.source_index == other.source_index
            && self.target_index == other.target_index
            && self.mode == other.mode
            && self.executable as usize == other.executable as usize
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("mode", &self.mode)
            .field("source_index", &self.source_index)
            .field("target_index", &self.target_index)
            .finish()
    }
}

impl Mutate for Instruction {
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

impl Show for Instruction {}

impl Instruction {
    fn get_target_data<'b, T>(&self, registers: Registers, data: &'b T) -> Registers
    where
        T: ValidInput,
    {
        let target_data: Registers = match self.mode {
            Mode::Internal => registers,
            Mode::External => data.into(),
        };

        target_data
    }

    pub fn apply<'b, T>(&self, registers: &'b mut Registers, input: &'b T)
    where
        T: ValidInput,
    {
        let cloned_registers = registers.clone();
        let data = self.get_target_data(cloned_registers, input);
        let target_value = *data.get(self.target_index);
        let source_value = *registers.get(self.source_index);
        let new_source_value = (self.executable)(source_value, target_value);
        registers.update(self.source_index, new_source_value);
    }
}
