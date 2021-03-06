use derivative::Derivative;
use derive_new::new;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use rand::prelude::SliceRandom;
use serde::Serialize;

use crate::{
    core::{
        characteristics::Fitness,
        inputs::ValidInput,
        program::Program,
        registers::{Registers, R32},
    },
    utils::random::generator,
};

use super::core::ExtensionParameters;

#[derive(Debug, Serialize, Derivative, new)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ReinforcementLearningParameters<T>
where
    T: ReinforcementLearningInput,
{
    pub n_runs: usize,
    pub max_episode_length: usize,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    pub environment: T,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Reward {
    Continue(R32),
    Terminal(R32),
}

#[derive(Debug, Clone)]
pub struct StateRewardPair {
    pub state: Vec<R32>,
    pub reward: Reward,
}

impl StateRewardPair {
    pub fn get_value(&self) -> R32 {
        match self.reward {
            Reward::Continue(reward) => reward,
            Reward::Terminal(reward) => reward,
        }
    }

    pub fn is_terminal(&self) -> bool {
        match self.reward {
            Reward::Continue(_) => false,
            Reward::Terminal(_) => true,
        }
    }
}

pub trait ReinforcementLearningInput: ValidInput + Sized {
    fn init(&mut self);
    fn act(&mut self, action: usize) -> StateRewardPair;
    fn reset(&mut self);
    fn get_state(&self) -> Vec<R32>;
    fn finish(&mut self);
}

impl<T> ExtensionParameters for ReinforcementLearningParameters<T>
where
    T: ReinforcementLearningInput,
{
    fn argmax(registers: &Registers) -> i32 {
        let action_registers = &registers[0..T::N_ACTION_REGISTERS];
        let max_value = action_registers
            .into_iter()
            .copied()
            .reduce(|a, b| f32::max(a, b))
            .unwrap();

        let indices = action_registers
            .into_iter()
            .enumerate()
            .filter(|(_, value)| **value == max_value)
            .map(|(index, _)| index)
            .collect_vec();

        indices.choose(&mut generator()).map(|v| *v as i32).unwrap()
    }
}

impl<T> Fitness for Program<ReinforcementLearningParameters<T>>
where
    T: ReinforcementLearningInput,
{
    type FitnessParameters = ReinforcementLearningParameters<T>;

    fn eval_fitness(
        &mut self,
        parameters: &mut Self::FitnessParameters,
    ) -> crate::core::characteristics::FitnessScore {
        let mut scores = vec![];

        parameters.environment.init();

        for _ in 0..parameters.n_runs {
            let mut score = 0.;

            for _ in 0..parameters.max_episode_length {
                // Run program.
                self.exec(&parameters.environment);
                // Eval
                let picked_action = ReinforcementLearningParameters::<T>::argmax(&self.registers);
                let state_reward = parameters.environment.act(picked_action as usize);

                score += state_reward.get_value();

                if state_reward.is_terminal() {
                    break;
                }
            }

            scores.push(score);
            parameters.environment.reset();
        }

        scores.sort_by(|a, b| a.partial_cmp(b).unwrap());
        parameters.environment.finish();

        let median = scores.remove(parameters.n_runs / 2);

        self.fitness = Some(median);

        median
    }

    fn get_fitness(&self) -> Option<crate::core::characteristics::FitnessScore> {
        self.fitness
    }
}

#[derive(Clone, Debug)]
pub struct QTable {
    table: Vec<Vec<R32>>,
    /// Step size parameter.
    alpha: R32,
    /// Discount.
    gamma: R32,
}

pub struct QProgram<T>
where
    T: ReinforcementLearningInput,
{
    program: Program<ReinforcementLearningParameters<T>>,
    q_table: QTable,
}

impl QTable {
    pub fn new(n_actions: usize, n_registers: usize, alpha: R32, gamma: R32) -> Self {
        let table = vec![vec![0.; n_actions]; n_registers];
        QTable {
            table,
            alpha,
            gamma,
        }
    }

    pub fn action_argmax(&self, register_number: usize) -> usize {
        let QTable { table, .. } = &self;
        let mut best_action = -1 as i32;
        let mut best_q_value = 0f32;
        let available_actions = table
            .get(register_number)
            .expect("Register number to be less than length of QTable.");

        for (action, q_value) in available_actions.into_iter().enumerate() {
            if *q_value > best_q_value {
                best_q_value = *q_value;
                best_action = action as i32;
            }
        }

        best_action as usize
    }

    pub fn update(
        &mut self,
        current_register: usize,
        current_action: usize,
        current_reward: R32,
        next_register: usize,
    ) {
        let current_q_value = self.table[current_register][current_action];
        let next_q_value = self.action_argmax(next_register) as f32;

        let new_q_value = current_q_value
            + self.alpha * (current_reward + self.gamma * next_q_value - current_q_value);

        self.table[current_register][current_action] = new_q_value;
    }
}

impl<T> Fitness for QProgram<T>
where
    T: ReinforcementLearningInput,
{
    type FitnessParameters = ReinforcementLearningParameters<T>;

    fn eval_fitness(
        &mut self,
        parameters: &mut Self::FitnessParameters,
    ) -> crate::core::characteristics::FitnessScore {
        parameters.environment.init();
        for _run in 0..parameters.n_runs {
            let mut score = 0f32;
            for _step in 0..parameters.max_episode_length {
                self.program.exec(&parameters.environment);

                let selected_register = self
                    .program
                    .registers
                    .iter()
                    .map(|v| OrderedFloat(*v))
                    .position_max()
                    .expect("Registers length to be greater than 0.");

                let state_reward_pair = parameters.environment.act(selected_register);

                score += state_reward_pair.get_value();

                if state_reward_pair.is_terminal() {
                    break;
                }
            }
        }

        todo!()
    }

    fn get_fitness(&self) -> Option<crate::core::characteristics::FitnessScore> {
        todo!()
    }
}
