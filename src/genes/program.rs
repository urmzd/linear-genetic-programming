use std::fmt::Display;

use crate::{
    metrics::definitions::Metric,
    utils::{
        common_traits::{Compare, Show, ValidInput},
        linked_list::LinkedList,
        random::generator,
    },
};
use rand::{prelude::IteratorRandom, Rng};
use serde::Serialize;

use crate::{
    metrics::accuracy::Accuracy,
    utils::common_traits::{Executables, Inputs},
};

use super::{
    characteristics::{Breed, Fitness, FitnessScore, Generate, Mutate, Organism},
    instruction::{Instruction, InstructionGenerateParams},
    registers::Registers,
};

pub type Instructions<'a> = LinkedList<Instruction<'a>>;

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

#[derive(Clone, Debug, Serialize)]
pub struct ProgramGenerateParams<'a, InputType>
where
    InputType: ValidInput,
{
    inputs: &'a Inputs<InputType>,
    max_instructions: usize,
    #[serde(skip_serializing)]
    instruction_generate_params: InstructionGenerateParams,
}

impl<'a, InputType> ProgramGenerateParams<'a, InputType>
where
    InputType: ValidInput,
{
    pub fn new(
        inputs: &'a Inputs<InputType>,
        max_instructions: usize,
        executables: Executables,
        register_len: Option<usize>,
    ) -> ProgramGenerateParams<'a, InputType> {
        let new_registers_len = register_len
            .or_else(|| Some(InputType::N_CLASSES + 1))
            .unwrap();

        let instruction_generate_params =
            InstructionGenerateParams::new(new_registers_len, InputType::N_FEATURES, executables);

        ProgramGenerateParams {
            inputs,
            max_instructions,
            instruction_generate_params,
        }
    }
}

impl<'a, InputType> Show for ProgramGenerateParams<'a, InputType> where InputType: ValidInput {}

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
    type GenerateParamsType = ProgramGenerateParams<'a, InputType>;

    fn generate(parameters: &'a Self::GenerateParamsType) -> Self {
        let ProgramGenerateParams {
            max_instructions,
            inputs,
            instruction_generate_params,
        } = &parameters;

        let registers = Registers::new(InputType::N_CLASSES + 1);

        let n_instructions = generator().gen_range(0..max_instructions.clone());

        let instructions: Instructions = (0..n_instructions)
            .map(|_| Instruction::generate(instruction_generate_params))
            .collect();

        Program {
            instructions,
            registers,
            inputs,
            fitness: None,
        }
    }
}

impl<'a, InputType> Fitness for Program<'a, InputType>
where
    InputType: ValidInput,
{
    fn eval_fitness(&self) -> FitnessScore {
        let inputs = self.inputs;

        let mut fitness: Accuracy<Option<usize>> = Accuracy::new();

        for input in inputs {
            let mut registers = self.registers.clone();

            for instruction in &self.instructions {
                instruction.apply(&mut registers, input);
            }

            let correct_index = input.get_class();
            let registers_argmax = registers.argmax(InputType::N_CLASSES, correct_index);

            Accuracy::observe(&mut fitness, [registers_argmax, Some(correct_index)]);

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
    InputType: ValidInput,
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
            registers: self.registers.clone(),
        };

        let program_b = Program {
            inputs: &self.inputs,
            instructions: child_b_instructions,
            fitness: None,
            registers: self.registers.clone(),
        };

        [program_a, program_b]
    }
}

impl<'a> Breed for Instructions<'a> {
    fn two_point_crossover(&self, mate: &Self) -> [Self; 2] {
        let mut instructions_a = self.clone();
        let mut instructions_b = mate.clone();

        let a_start = generator().gen_range(0..instructions_a.len());
        let a_end = if a_start == instructions_a.len() {
            None
        } else {
            Some(generator().gen_range(a_start..=instructions_a.len())).and_then(|index| {
                if index == instructions_a.len() || a_start == index {
                    None
                } else {
                    Some(index)
                }
            })
        };

        let b_start = generator().gen_range(0..instructions_b.len());
        let b_end = if b_start == instructions_b.len() {
            None
        } else {
            Some(generator().gen_range(b_start..=instructions_b.len())).and_then(|index| {
                if index == instructions_b.len() || b_start == index {
                    None
                } else {
                    Some(index)
                }
            })
        };

        let mut cursor_a = instructions_a.cursor_mut();
        let mut cursor_b = instructions_b.cursor_mut();

        cursor_a.swap(&mut cursor_b, a_start, b_start, a_end, b_end);

        [instructions_a, instructions_b]
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::iris::ops::IRIS_EXECUTABLES, utils::test::TestInput};

    use super::*;

    #[test]
    fn given_instructions_when_breed_then_two_children_are_produced_using_genes_of_parents() {
        let params_a = InstructionGenerateParams::new(5, 5, IRIS_EXECUTABLES);
        let params_b = InstructionGenerateParams::new(6, 6, IRIS_EXECUTABLES);
        let instructions_a: Instructions =
            (0..5).map(|_| Instruction::generate(&params_a)).collect();
        let instructions_b: Instructions =
            (0..5).map(|_| Instruction::generate(&params_b)).collect();

        let [child_a, child_b] = instructions_a.two_point_crossover(&instructions_b);

        assert_ne!(instructions_a, child_a);
        assert_ne!(instructions_b, child_a);

        assert_ne!(instructions_a, child_b);
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

        let program_params = ProgramGenerateParams::new(&inputs, 100, IRIS_EXECUTABLES, None);

        let program_a = Program::generate(&program_params);
        let program_b = Program::generate(&program_params);

        let [child_a, child_b] = program_a.two_point_crossover(&program_b);

        assert_ne!(program_a, child_a);
        assert_ne!(program_a, child_a);

        assert_ne!(program_b, child_b);
        assert_ne!(program_b, child_b);
    }
}
