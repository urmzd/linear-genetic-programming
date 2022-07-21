use std::{fmt::Display, marker::PhantomData};

use crate::utils::random::generator;
use derivative::Derivative;
use derive_new::new;
use rand::{
    distributions::Uniform,
    prelude::{Distribution, IteratorRandom},
};
use serde::Serialize;

use super::{
    characteristics::{Breed, FitnessScore, Generate, Mutate},
    inputs::ValidInput,
    instruction::{Instruction, InstructionGeneratorParameters},
    instructions::Instructions,
    registers::Registers,
};
#[derive(Clone, Debug, Serialize, new)]
pub struct ProgramGeneratorParameters {
    pub max_instructions: usize,
    pub instruction_generator_parameters: InstructionGeneratorParameters,
}

impl<T> Clone for Program<T> {
    fn clone(&self) -> Self {
        Self {
            instructions: self.instructions.clone(),
            registers: self.registers.clone(),
            fitness: self.fitness.clone(),
            marker: self.marker.clone(),
        }
    }
}

#[derive(Debug, Serialize, new, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Program<T> {
    #[derivative(Ord = "ignore", PartialOrd = "ignore")]
    pub instructions: Instructions,
    #[derivative(Ord = "ignore", PartialOrd = "ignore")]
    pub registers: Registers,
    pub fitness: Option<FitnessScore>,
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    marker: PhantomData<T>,
}

impl<T> Program<T> {
    pub fn exec<I>(&mut self, input: &I)
    where
        I: ValidInput,
    {
        for instruction in &self.instructions {
            instruction.apply(&mut &mut self.registers, input)
        }
    }
}

impl<T> Display for Program<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = toml::to_string(&self).unwrap();
        f.write_str(&serialized)
    }
}

impl<T> Generate for Program<T> {
    type GeneratorParameters = ProgramGeneratorParameters;

    fn generate<'a>(parameters: &'a Self::GeneratorParameters) -> Self {
        let ProgramGeneratorParameters {
            max_instructions,
            instruction_generator_parameters,
        } = &parameters;

        let registers = Registers::new(instruction_generator_parameters.n_registers);
        let n_instructions = Uniform::new_inclusive(1, max_instructions).sample(&mut generator());
        let instructions = (0..n_instructions)
            .into_iter()
            .map(|_| Instruction::generate(instruction_generator_parameters))
            .collect();

        Self::new(instructions, registers, None)
    }
}

impl<T> Mutate for Program<T> {
    fn mutate(&self, params: &Self::GeneratorParameters) -> Self {
        let mut mutated = self.clone();

        // Pick instruction to mutate.
        let instruction = mutated
            .instructions
            .iter_mut()
            .choose(&mut generator())
            .unwrap();

        let mutated_instruction = instruction.mutate(&params.instruction_generator_parameters);
        *instruction = mutated_instruction;

        // IMPORTANT: Reset fitness to force evaluation.
        mutated.fitness = None;

        mutated
    }
}

impl<T> Breed for Program<T> {
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2] {
        let [child_a_instructions, child_b_instructions] =
            self.instructions.two_point_crossover(&mate.instructions);

        let program_a = Program::new(child_a_instructions, self.registers.duplicate(), None);

        let program_b = Program::new(child_b_instructions, self.registers.duplicate(), None);

        [program_a, program_b]
    }
}

#[cfg(test)]
mod tests {
    use rand::{distributions::Standard, Rng};

    use crate::{
        core::instruction::InstructionGeneratorParameters,
        extensions::classification::ClassificationParameters, utils::random::generator,
        utils::test::TestInput,
    };

    use super::*;

    #[test]
    fn given_instructions_when_breed_then_two_children_are_produced_using_genes_of_parents() {
        let params = InstructionGeneratorParameters::new(5, 5);
        let instructions_a: Instructions =
            (0..10).map(|_| Instruction::generate(&params)).collect();
        let instructions_b: Instructions =
            (0..10).map(|_| Instruction::generate(&params)).collect();

        let [child_a, child_b] = instructions_a.two_point_crossover(&instructions_b);

        assert_ne!(child_a, child_b);

        assert_ne!(instructions_a, child_a);
        assert_ne!(instructions_a, child_b);

        assert_ne!(instructions_b, child_a);
        assert_ne!(instructions_b, child_b);
    }

    #[test]
    fn given_programs_when_two_point_crossover_then_two_children_are_produced() {
        let inputs: Vec<TestInput> = [0; 5].map(|_| generator().sample(Standard)).to_vec();

        let instruction_params = InstructionGeneratorParameters::new(3, 4);
        let classification_params = ClassificationParameters::new(inputs);
        let program_params = ProgramGeneratorParameters::new(100, instruction_params);

        let program_a = Program::<TestInput>::generate(&program_params);
        let program_b = Program::<TestInput>::generate(&program_params);

        let [child_a, child_b] = program_a.two_point_crossover(&program_b);

        assert_ne!(child_a, child_b);

        assert_ne!(program_a, child_a);
        assert_ne!(program_a, child_b);

        assert_ne!(program_b, child_a);
        assert_ne!(program_b, child_b);
    }
}
