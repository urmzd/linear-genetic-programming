use std::fmt::Display;

use crate::{
    metrics::definitions::Metric,
    utils::{
        common_traits::{Compare, Show, ValidInput},
        problem_types::ClassificationProblem,
        random::generator,
    },
};
use more_asserts::assert_ge;
use rand::{prelude::IteratorRandom, Rng};
use serde::Serialize;

use crate::{
    metrics::accuracy::Accuracy,
    utils::common_traits::{Executables, Inputs},
};

use super::{
    characteristics::{Breed, Fitness, FitnessScore, Generate, Mutate, Organism},
    instruction::{Instruction, InstructionGeneratorParameters},
    instructions::Instructions,
    registers::Registers,
};

#[derive(Clone, Debug, Serialize)]
pub struct ProgramGeneratorParameters<'a, T> {
    max_instructions: usize,
    n_registers: usize,
    task_specific: T,
    instructions: Instructions,
}

#[derive(Clone, Debug, Serialize)]
struct ClassificationSpecificParameters<'a, InputType> {
    inputs: &'a Inputs<InputType>,
}

impl<'a, InputType> Show for ProgramGeneratorParameters<'a, InputType> where InputType: ValidInput {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Program<'a, InputType>
where
    InputType: ValidInput,
{
    pub instructions: Instructions<'a>,
    pub inputs: &'a Inputs<InputType>,
    pub registers: Registers,
    fitness: Option<FitnessScore>,
}

impl<'a, InputType> Display for Program<'a, InputType>
where
    InputType: ValidInput,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serialized = toml::to_string(&self).unwrap();
        f.write_str(&serialized)
    }
}

impl<'a, InputType> Ord for Program<'a, InputType>
where
    InputType: ValidInput,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fitness.cmp(&other.fitness)
    }
}

impl<'a, InputType> PartialOrd for Program<'a, InputType>
where
    InputType: ValidInput,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl<'a, InputType> Generate<'a> for Program<'a, InputType>
where
    InputType: ValidInput,
{
    type GenerateParamsType = ProgramGeneratorParameters;

    fn generate(parameters: &'a Self::GenerateParamsType) -> Self {
        let ProgramGeneratorParameters {
            max_instructions,
            inputs,
            instructions,
            n_registers: registers_len,
        } = &parameters;

        let registers = Registers::new(registers_len.clone());

        Program {
            instructions,
            registers,
            inputs,
            fitness: None,
        }
    }
}

impl<'a, ProblemType> Fitness for Program<'a, ProblemType>
where
    ProblemType: ClassificationProblem,
{
    fn eval_fitness(&self) -> FitnessScore {
        let inputs = self.inputs;

        let mut fitness: Accuracy<Option<ProblemType::Represent>> = Accuracy::new();

        for input in inputs {
            let mut registers = self.registers.clone();

            for instruction in &self.instructions {
                instruction.apply(&mut registers, input);
            }

            let argmax = input.argmax(&registers);
            let correct_class = input.get_class();

            fitness.observe([argmax, Some(correct_class)]);

            registers.reset();
        }

        fitness.calculate()
    }

    fn eval_set_fitness(&mut self) -> FitnessScore {
        *self.fitness.get_or_insert(self.eval_fitness())
    }

    fn get_fitness(&self) -> Option<FitnessScore> {
        self.fitness
    }
}

impl<'a, InputType> Show for Program<'a, InputType> where InputType: ValidInput {}

impl<'a, InputType> Compare for Program<'a, InputType> where InputType: ValidInput {}

impl<'a, InputType> Organism<'a> for Program<'a, InputType>
where
    InputType: ClassificationProblem,
{
    fn get_instructions(&self) -> &Instructions {
        &self.instructions
    }
}

impl<'a, InputType> Mutate for Program<'a, InputType>
where
    InputType: ValidInput,
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

        // Reset fitness to force evaluation.
        mutated.fitness = None;

        mutated
    }
}

impl<'a, InputType> Breed for Program<'a, InputType>
where
    InputType: ValidInput,
{
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2] {
        let [child_a_instructions, child_b_instructions] =
            self.instructions.two_point_crossover(&mate.instructions);

        let program_a = Program {
            inputs: &self.inputs,
            instructions: child_a_instructions,
            fitness: None,
            registers: self.registers.clone().reset().to_owned(),
        };

        let program_b = Program {
            inputs: &self.inputs,
            instructions: child_b_instructions,
            fitness: None,
            registers: self.registers.clone().reset().to_owned(),
        };

        [program_a, program_b]
    }
}

#[cfg(test)]
mod tests {
    use crate::{examples::iris::ops::IRIS_EXECUTABLES, utils::test::TestInput};

    use super::*;

    #[test]
    fn given_instructions_when_breed_then_two_children_are_produced_using_genes_of_parents() {
        let params_a = InstructionGeneratorParameters::new(5, 5, IRIS_EXECUTABLES);
        let params_b = InstructionGeneratorParameters::new(6, 6, IRIS_EXECUTABLES);
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

        let program_params = ProgramGeneratorParameters::new(&inputs, 100, IRIS_EXECUTABLES, 4);

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
