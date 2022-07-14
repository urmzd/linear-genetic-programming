use std::{fmt::Display, marker::PhantomData};

use crate::utils::{
    common_traits::{Compare, Show, ValidInput},
    random::generator,
};
use derive_new::new;
use rand::{
    distributions::Uniform,
    prelude::{Distribution, IteratorRandom},
};
use serde::Serialize;

use super::{
    characteristics::{Breed, FitnessScore, Generate, Mutate},
    instruction::{Instruction, InstructionGeneratorParameters},
    instructions::Instructions,
    registers::{RegisterGeneratorParameters, Registers},
};
pub trait ExtensionParameters<'a>
where
    Self::InputType: ValidInput,
{
    type InputType;
}

#[derive(Clone, Debug, Serialize, new)]
pub struct ProgramGeneratorParameters<'a, OtherParameters>
where
    OtherParameters: ExtensionParameters<'a>,
{
    pub max_instructions: usize,
    pub instruction_generator_parameters:
        InstructionGeneratorParameters<'a, OtherParameters::InputType>,
    pub register_generator_parameters: RegisterGeneratorParameters,
    pub other: OtherParameters,
    marker: PhantomData<&'a ()>,
}

impl<'a, T> Show for ProgramGeneratorParameters<'a, T> where T: Show + ExtensionParameters<'a> {}

#[derive(Debug, Serialize)]
pub struct Program<'a, T>
where
    T: ExtensionParameters<'a>,
{
    pub instructions: Instructions<'a, T::InputType>,
    pub registers: Registers<'a>,
    pub fitness: Option<FitnessScore>,
    // Problem specific parameters
    pub other: &'a T,
}

impl<'a, T> Clone for Program<'a, T>
where
    T: ExtensionParameters<'a>,
{
    fn clone(&self) -> Self {
        Self {
            instructions: self.instructions.clone(),
            registers: self.registers.clone(),
            fitness: self.fitness.clone(),
            other: &self.other,
        }
    }
}

impl<'a, T> Display for Program<'a, T>
where
    T: Serialize + ExtensionParameters<'a>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = toml::to_string(&self).unwrap();
        f.write_str(&serialized)
    }
}

impl<'a, T> Ord for Program<'a, T>
where
    T: ExtensionParameters<'a>,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

impl<'a, T> PartialOrd for Program<'a, T>
where
    T: ExtensionParameters<'a>,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl<'a, T> PartialEq for Program<'a, T>
where
    T: ExtensionParameters<'a>,
{
    fn eq(&self, other: &Self) -> bool {
        self.instructions == other.instructions
            && self.registers == other.registers
            && self.fitness == other.fitness
    }
}

impl<'a, T> Eq for Program<'a, T> where T: ExtensionParameters<'a> {}

impl<'a, T> Generate<'a> for Program<'a, T>
where
    T: ExtensionParameters<'a>,
{
    type GeneratorParameters = ProgramGeneratorParameters<'a, T>;

    fn generate(parameters: &'a Self::GeneratorParameters) -> Self {
        let ProgramGeneratorParameters {
            max_instructions,
            instruction_generator_parameters,
            register_generator_parameters,
            other,
            ..
        } = &parameters;

        let registers = Registers::generate::<T::InputType>(register_generator_parameters);
        let n_instructions = Uniform::new_inclusive(1, max_instructions).sample(&mut generator());
        let instructions = (0..n_instructions)
            .into_iter()
            .map(|_| Instruction::generate(instruction_generator_parameters))
            .collect();

        Program {
            instructions,
            registers,
            other,
            fitness: None,
        }
    }
}

impl<'a, T> Show for Program<'a, T> where T: Show + ExtensionParameters<'a> {}

impl<'a, T> Compare for Program<'a, T> where T: Compare + ExtensionParameters<'a> {}

impl<'a, T> Mutate for Program<'a, T>
where
    T: ExtensionParameters<'a>,
{
    fn mutate(&self) -> Self {
        let mut mutated = self.clone();

        // pick instruction to mutate.
        let instruction = mutated
            .instructions
            .iter_mut()
            .choose(&mut generator())
            .unwrap();

        let mutated_instruction = instruction.mutate();
        *instruction = mutated_instruction;

        // IMPORTANT: Reset fitness to force evaluation.
        mutated.fitness = None;

        mutated
    }
}

impl<'a, T> Breed for Program<'a, T>
where
    T: ExtensionParameters<'a>,
{
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2] {
        let [child_a_instructions, child_b_instructions] =
            self.instructions.two_point_crossover(&mate.instructions);

        let program_a = Program {
            other: self.other,
            instructions: child_a_instructions,
            fitness: None,
            registers: self.registers.clone().reset().to_owned(),
        };

        let program_b = Program {
            other: self.other,
            instructions: child_b_instructions,
            fitness: None,
            registers: self.registers.clone().reset().to_owned(),
        };

        [program_a, program_b]
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::instruction::InstructionGeneratorParameters,
        extensions::classification::ClassificationParameters, utils::test::TestInput,
    };

    use super::*;

    #[test]
    fn given_instructions_when_breed_then_two_children_are_produced_using_genes_of_parents() {
        let params = InstructionGeneratorParameters::new(5, 5);
        let instructions_a: Instructions<TestInput> =
            (0..5).map(|_| Instruction::generate(&params)).collect();
        let instructions_b: Instructions<TestInput> =
            (0..5).map(|_| Instruction::generate(&params)).collect();

        let [child_a, child_b] = instructions_a.two_point_crossover(&instructions_b);

        assert_ne!(child_a, child_b);

        assert_ne!(instructions_a, child_a);
        assert_ne!(instructions_a, child_b);

        assert_ne!(instructions_b, child_a);
        assert_ne!(instructions_b, child_b);
    }

    #[test]
    fn given_programs_when_two_point_crossover_then_two_children_are_produced() {
        let inputs = [
            TestInput::new([0; 5]),
            TestInput::new([1; 5]),
            TestInput::new([0, 0, 0, 1, 0]),
            TestInput::new([1, 0, 1, 1, 1]),
        ]
        .to_vec();

        let instruction_params = InstructionGeneratorParameters::new(3, 4);
        let classification_params = ClassificationParameters::new(&inputs);
        let register_params = RegisterGeneratorParameters::new(2);
        let program_params = ProgramGeneratorParameters::new(
            100,
            instruction_params,
            register_params,
            classification_params,
        );

        let program_a = Program::generate(&program_params);
        let program_b = Program::generate(&program_params);

        let [child_a, child_b] = program_a.two_point_crossover(&program_b);

        assert_ne!(child_a, child_b);

        assert_ne!(program_a, child_a);
        assert_ne!(program_a, child_b);

        assert_ne!(program_b, child_a);
        assert_ne!(program_b, child_b);
    }
}
