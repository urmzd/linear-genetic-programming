use derive_new::new;
use num_derive::FromPrimitive;
use rand::distributions::uniform::{UniformInt, UniformSampler};
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use serde::Serialize;
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::marker::PhantomData;
use strum::EnumCount;

use crate::utils::executables::AnyExecutable;
use crate::utils::random::generator;

use super::characteristics::{Generate, Mutate, Show};
use super::inputs::ValidInput;
use super::registers::Registers;

#[derive(FromPrimitive, Clone, Debug, EnumCount, PartialEq, Eq, Serialize)]
pub enum Mode {
    External = 0,
    Internal = 1,
}

impl<'a, T> Show for InstructionGeneratorParameters<'a, T> where T: Show + ValidInput {}
#[derive(Clone, Debug, Serialize, new)]
pub struct InstructionGeneratorParameters<'a, T>
where
    T: ValidInput,
{
    pub n_registers: usize,
    pub n_features: usize,
    marker: PhantomData<&'a T>,
}

impl<'a, T> InstructionGeneratorParameters<'a, T>
where
    T: ValidInput,
{
    pub fn from() -> Self {
        InstructionGeneratorParameters::new(
            <T as ValidInput>::Actions::COUNT,
            <T as ValidInput>::N_INPUTS,
        )
    }
}

pub type Modes = &'static [Mode];

impl Mode {
    pub const ALL: Modes = &[Mode::Internal, Mode::External];
    pub const INTERNAL_ONLY: Modes = &[Mode::Internal];
    pub const EXTERNAL_ONLY: Modes = &[Mode::External];
}

#[derive(Serialize)]
pub struct Instruction<'a, T>
where
    T: ValidInput,
{
    source_index: usize,
    target_index: usize,
    mode: Mode,
    #[serde(skip_serializing)]
    executable: AnyExecutable,
    parameters_used: &'a InstructionGeneratorParameters<'a, T>,
}

impl<'a, T> Clone for Instruction<'a, T>
where
    T: ValidInput,
{
    fn clone(&self) -> Self {
        Self {
            source_index: self.source_index.clone(),
            target_index: self.target_index.clone(),
            mode: self.mode.clone(),
            executable: self.executable.clone(),
            parameters_used: &self.parameters_used,
        }
    }
}

impl<'a, T> Generate<'a> for Instruction<'a, T>
where
    T: ValidInput,
{
    type GeneratorParameters = InstructionGeneratorParameters<'a, T>;

    fn generate(parameters: &'a Self::GeneratorParameters) -> Self {
        let InstructionGeneratorParameters {
            n_features: n_inputs,
            n_registers,
            ..
        } = parameters;

        let source_index = UniformInt::<usize>::new(0, n_registers).sample(&mut generator());

        let mode = T::AVAILABLE_MODES.choose(&mut generator()).unwrap().clone();

        let upper_bound_target_index = *(if mode == Mode::External {
            n_inputs
        } else {
            n_registers
        });
        let target_index =
            UniformInt::<usize>::new(0, upper_bound_target_index).sample(&mut thread_rng());

        let exec = T::AVAILABLE_EXECUTABLES
            .choose(&mut generator())
            .unwrap()
            .to_owned();

        Instruction {
            source_index,
            target_index,
            executable: exec,
            mode,
            parameters_used: &parameters,
        }
    }
}

impl<'b, T> Eq for Instruction<'b, T> where T: ValidInput {}

impl<'b, T> PartialEq for Instruction<'b, T>
where
    T: ValidInput,
{
    fn eq(&self, other: &Self) -> bool {
        self.source_index == other.source_index
            && self.target_index == other.target_index
            && self.mode == other.mode
            && self.executable.get_fn() as usize == other.executable.get_fn() as usize
    }
}

impl<'b, T> Debug for Instruction<'b, T>
where
    T: ValidInput,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("mode", &self.mode)
            .field("source_index", &self.source_index)
            .field("target_index", &self.target_index)
            .finish()
    }
}

impl<'b, T> Mutate for Instruction<'b, T>
where
    T: ValidInput,
{
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

impl<'b, T> Show for Instruction<'b, T> where T: ValidInput {}

impl<'b, T> Instruction<'b, T>
where
    T: ValidInput,
{
    fn get_target_data(&self, registers: &'b Registers, data: &'b T) -> Registers<'b> {
        let target_data: Registers<'b> = match self.mode {
            Mode::Internal => registers.clone(),
            Mode::External => data.into(),
        };

        target_data
    }

    pub fn apply(&self, registers: &'b mut Registers, input: &'b T) {
        let cloned_registers = registers.clone();
        let data = self.get_target_data(&cloned_registers, input);
        let target_slice = data.as_slice(self.target_index, None);
        let source_slice = registers.as_mut_slice(self.source_index, None);
        (self.executable.get_fn())(source_slice, target_slice);
    }
}
