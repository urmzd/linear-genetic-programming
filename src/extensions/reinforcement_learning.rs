use std::{collections::HashMap, marker::PhantomData};

use derivative::Derivative;
use derive_new::new;
use serde::Serialize;

use crate::core::{
    characteristics::Fitness, inputs::ValidInput, program::Program, registers::RegisterValue,
};

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
    Continue(RegisterValue),
    Terminal(RegisterValue),
}

struct QTable(HashMap<usize, Vec<usize>>);

impl QTable {
    pub fn argmax() {}

    pub fn update(&mut self) {}
}

trait QLearning<S, A> {
    fn eval(&mut self, state: S, q_table: &mut QTable) -> A;
    fn sim(&mut self, action: A) -> StateRewardPair;
    fn update(&mut self, q_table: &mut QTable);
}

#[derive(Debug, Clone)]
pub struct StateRewardPair {
    pub state: Vec<RegisterValue>,
    pub reward: Reward,
}

impl StateRewardPair {
    pub fn get_value(&self) -> RegisterValue {
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
    fn act(&mut self, action: Self::Actions) -> StateRewardPair;
    fn reset(&mut self);
    fn get_state(&self) -> Vec<RegisterValue>;
    fn finish(&mut self);
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
        todo!()
    }

    fn get_fitness(&self) -> Option<crate::core::characteristics::FitnessScore> {
        todo!()
    }
}

// impl<'a, T> Fitness for Program<'a, T>
// where
//     T: ReinforcementLearningInput,
// {
//     // fn eval_fitness(&mut self) -> crate::core::characteristics::FitnessScore {
//     //     let mut scores = vec![];

//     //     let ReinforcementLearningParameters {
//     //         n_runs,
//     //         max_episode_length,
//     //         mut environment,
//     //         ..
//     //     } = self.problem_parameters.clone();

//     //     environment.init();

//     //     for _ in 0..n_runs {
//     //         let mut score = OrderedFloat(0.);

//     //         for _ in 0..max_episode_length {
//     //             // Run program.
//     //             self.exec(&environment);
//     //             // Eval
//     //             let possible_actions = T::argmax(&self.registers);
//     //             let state_reward = environment.act(selected_action);

//     //             score += state_reward.get_value();

//     //             if state_reward.is_terminal() {
//     //                 break;
//     //             }
//     //         }

//     //         scores.push(score);
//     //         environment.reset();
//     //     }

//     //     scores.sort();
//     //     environment.finish();

//     //     let median = scores.remove(n_runs / 2);

//     //     self.fitness = Some(median);

//     //     median
//     // }

//     // fn get_fitness(&self) -> Option<crate::core::characteristics::FitnessScore> {
//     //     self.fitness
//     // }

//     type FitnessParams = usize;

//     fn eval_fitness(
//         &mut self,
//         params: Self::FitnessParams,
//     ) -> crate::core::characteristics::FitnessScore {
//         todo!()
//     }

//     fn get_fitness(&self) -> Option<crate::core::characteristics::FitnessScore> {
//         todo!()
//     }
// }

// impl<'a, T> Organism for Program<T> where T: ReinforcementLearningInput {}
// impl<'a, T> Organism for Program<T> where T: ClassificationInput {}

struct G<T>(PhantomData<T>);
