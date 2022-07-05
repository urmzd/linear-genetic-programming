use std::ops::{Add, AddAssign, Div};

use derive_new::new;
use serde::Serialize;

use crate::{
    core::{
        characteristics::{Fitness, FitnessScore, Organism},
        program::{ExtensionParameters, Program},
    },
    utils::common_traits::{Compare, Show, ValidInput},
};

#[derive(Debug, Clone, new, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub struct ReinforcementLearningParameters<T>
where
    T: ReinforcementLearningInput,
{
    n_runs: usize,
    environment: T,
}
pub enum Reward<RewardValue> {
    Continue(RewardValue),
    Terminal(RewardValue),
}

impl<T> Reward<T>
where
    T: Copy,
{
    pub fn get_reward_value(&self) -> T {
        *(match self {
            Self::Continue(reward) => reward,
            Self::Terminal(reward) => reward,
        })
    }

    pub fn should_stop(&self) -> bool {
        match self {
            Self::Continue(_) => false,
            Self::Terminal(_) => true,
        }
    }
}

impl<T> ExtensionParameters for ReinforcementLearningParameters<T>
where
    T: ReinforcementLearningInput,
{
    type InputType = T;
}

pub trait ReinforcementLearningInput: ValidInput
where
    Self::RewardValue: Default
        + Add<Self::RewardValue>
        + AddAssign<Self::RewardValue>
        + Copy
        + Div<usize, Output = FitnessScore>,
{
    type RewardValue;

    fn init(&mut self);
    fn act(&mut self, action: Self::Actions) -> Reward<Self::RewardValue>;
    fn finish(&mut self);
}

impl<'a, T> Fitness for Program<'a, ReinforcementLearningParameters<T>>
where
    T: ReinforcementLearningInput,
{
    fn eval_fitness(&self) -> crate::core::characteristics::FitnessScore {
        let mut scores = vec![];
        let ReinforcementLearningParameters {
            n_runs,
            environment: mut game,
        } = self.other.clone();
        for _ in 0..n_runs {
            let mut registers = self.registers.clone();
            let mut score = T::RewardValue::default();

            game.init();

            for instruction in &self.instructions {
                let target_data = registers.clone();
                instruction.apply(&mut registers, &target_data);
                let possible_actions = registers.argmax();
                let selected_action = T::argmax(possible_actions).unwrap();
                let reward = game.act(selected_action);

                score += reward.get_reward_value();

                if reward.should_stop() {
                    break;
                }
            }

            game.finish();

            scores.push(score)
        }

        let average = scores
            .into_iter()
            .fold(FitnessScore::default(), |acc, current| {
                let current_weighted = current / n_runs;
                current_weighted + acc
            });

        average
    }

    fn eval_set_fitness(&mut self) -> crate::core::characteristics::FitnessScore {
        *self.fitness.get_or_insert(self.eval_fitness())
    }

    fn get_fitness(&self) -> Option<crate::core::characteristics::FitnessScore> {
        Some(self.eval_fitness())
    }
}

impl<'a, T> Organism<'a> for Program<'a, ReinforcementLearningParameters<T>> where
    T: ReinforcementLearningInput
{
}
impl<T> Show for ReinforcementLearningParameters<T> where T: ReinforcementLearningInput {}
impl<T> Compare for ReinforcementLearningParameters<T> where T: ReinforcementLearningInput {}
