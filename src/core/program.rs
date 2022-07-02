use std::fmt::Display;

use crate::utils::{
    common_traits::{Compare, Show},
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
    registers::Registers,
};

#[derive(Clone, Debug, Serialize, new)]
pub struct ProgramGeneratorParameters<T> {
    max_instructions: usize,
    instruction_generator_parameters: InstructionGeneratorParameters,
    other: T,
}

impl<T> Show for ProgramGeneratorParameters<T> where T: Show {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Program<'a, T> {
    pub instructions: Instructions<'a>,
    pub registers: Registers,
    pub fitness: Option<FitnessScore>,
    // Problem specific parameters
    pub other: &'a T,
}

impl<'a, T> Display for Program<'a, T>
where
    T: Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = toml::to_string(&self).unwrap();
        f.write_str(&serialized)
    }
}

impl<'a, T> Ord for Program<'a, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

impl<'a, T> PartialOrd for Program<'a, T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl<'a, T> Generate<'a> for Program<'a, T> {
    type GeneratorParameters = ProgramGeneratorParameters<T>;

    fn generate(parameters: &'a Self::GeneratorParameters) -> Self {
        let ProgramGeneratorParameters {
            max_instructions,
            instruction_generator_parameters,
            other,
        } = &parameters;

        let registers = Registers::new(instruction_generator_parameters.n_registers.clone());
        let n_instructions = Uniform::new_inclusive(0, max_instructions).sample(&mut generator());
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

impl<'a, T> Show for Program<'a, T> where T: Show {}

impl<'a, T> Compare for Program<'a, T> where T: Compare {}

impl<'a, T> Mutate for Program<'a, T>
where
    T: Clone,
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
    T: Clone,
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
        core::instruction::{InstructionGeneratorParameters, Modes},
        examples::iris::ops::IRIS_EXECUTABLES,
        problem_types::classification::Classification,
        utils::test::TestInput,
    };

    use super::*;

    #[test]
    fn given_instructions_when_breed_then_two_children_are_produced_using_genes_of_parents() {
        let params_a =
            InstructionGeneratorParameters::new(5, Some(5), Modes::all(), IRIS_EXECUTABLES);
        let params_b =
            InstructionGeneratorParameters::new(6, Some(6), Modes::all(), IRIS_EXECUTABLES);
        let instructions_a: Instructions =
            (0..5).map(|_| Instruction::generate(&params_a)).collect();
        let instructions_b: Instructions =
            (0..5).map(|_| Instruction::generate(&params_b)).collect();

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
            TestInput([0; 5]),
            TestInput([1; 5]),
            TestInput([0, 0, 0, 1, 0]),
            TestInput([1, 0, 1, 1, 1]),
        ]
        .to_vec();

        let instruction_params =
            InstructionGeneratorParameters::new(3, Some(5), Modes::all(), IRIS_EXECUTABLES);
        let classification_params = Classification::new(&inputs);
        let program_params =
            ProgramGeneratorParameters::new(100, instruction_params, classification_params);

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
