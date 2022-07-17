use derivative::Derivative;
use derive_new::new;
use ordered_float::OrderedFloat;
use serde::Serialize;

use crate::core::{
    characteristics::{Compare, Fitness, Organism, Show},
    inputs::ValidInput,
    program::{ExtensionParameters, Program},
    registers::RegisterValue,
};

#[derive(Debug, Serialize, Derivative, new)]
#[derivative(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ReinforcementLearningParameters<T>
where
    T: ReinforcementLearningInput,
{
    n_runs: usize,
    max_episodes: usize,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    environment: T,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Reward {
    Continue(RegisterValue),
    Terminal(RegisterValue),
}

impl Reward {
    pub fn get_reward_value(&self) -> RegisterValue {
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

impl<'a, T> ExtensionParameters for ReinforcementLearningParameters<T>
where
    T: ReinforcementLearningInput,
{
    type InputType = T;
}

pub trait ReinforcementLearningInput: ValidInput + Sized {
    fn init(&mut self);
    fn act(&mut self, action: Self::Actions) -> Reward;
    fn reset(&mut self);
    fn get_state(&self) -> Vec<RegisterValue>;
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
            max_episodes,
            mut environment,
            ..
        } = self.problem_parameters.clone();

        environment.init();

        for _ in 0..n_runs {
            let mut score = OrderedFloat(0.);

            for _ in 0..max_episodes {
                let mut registers = self.registers.clone();
                for instruction in &self.instructions {
                    instruction.apply(&mut registers, &environment);
                }
                let possible_actions = registers.argmax();
                let selected_action = T::argmax(possible_actions).unwrap();
                let reward = environment.act(selected_action);

                score += reward.get_reward_value();

                if reward.should_stop() {
                    break;
                }
            }

            scores.push(score);
            environment.reset();
        }

        scores.sort();
        environment.finish();

        let median = scores.remove(n_runs / 2);

        median.into()
    }

    fn get_or_eval_fitness(&mut self) -> crate::core::characteristics::FitnessScore {
        *self.fitness.get_or_insert(self.eval_fitness())
    }

    fn get_fitness(&self) -> Option<crate::core::characteristics::FitnessScore> {
        self.fitness
    }
}

impl<'a, T> Organism<'a> for Program<'a, ReinforcementLearningParameters<T>> where
    T: ReinforcementLearningInput + Show
{
}
impl<'a, T> Show for ReinforcementLearningParameters<T> where T: ReinforcementLearningInput + Show {}
impl<'a, T> Compare for ReinforcementLearningParameters<T> where T: ReinforcementLearningInput {}
