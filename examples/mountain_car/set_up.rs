use std::marker::PhantomData;

use derive_new::new;
use gym_rs::core::ActionReward;
use gym_rs::{core::Env, envs::classical_control::mountain_car::MountainCarEnv};
use lgp::extensions::reinforcement_learning::StateRewardPair;
use lgp::{
    core::{
        algorithm::GeneticAlgorithm, characteristics::Show, inputs::ValidInput, program::Program,
        registers::RegisterValue,
    },
    extensions::reinforcement_learning::{
        ReinforcementLearningInput, ReinforcementLearningParameters, Reward,
    },
    utils::executables::{Executables, DEFAULT_EXECUTABLES},
};
use num::NumCast;
use num_derive::{FromPrimitive, ToPrimitive};
use ordered_float::OrderedFloat;
use serde::Serialize;
use strum::{Display, EnumCount};

#[derive(Debug, Clone, Display, Eq, PartialEq, EnumCount, FromPrimitive, ToPrimitive)]
pub enum MountainCarActions {
    AccelerateLeft = 0,
    Pause = 1,
    AccelerateRight = 2,
}

pub struct MountainCarLgp<'a>(PhantomData<&'a ()>);

impl<'a> GeneticAlgorithm<'a> for MountainCarLgp<'a> {
    type O = Program<'a, ReinforcementLearningParameters<MountainCarInput<'a>>>;
}

#[derive(Debug, Serialize, new, Clone)]
pub struct MountainCarInput<'a> {
    environment: MountainCarEnv<'a>,
}

impl Show for MountainCarInput<'_> {}

impl ValidInput for MountainCarInput<'_> {
    type Actions = MountainCarActions;

    const N_INPUTS: usize = 2;

    const AVAILABLE_EXECUTABLES: Executables = DEFAULT_EXECUTABLES;

    fn flat(&self) -> Vec<RegisterValue> {
        let state = self.get_state();
        state
    }
}

impl ReinforcementLearningInput for MountainCarInput<'_> {
    fn init(&mut self) {
        self.environment.reset(None, false, None);
    }

    fn act(&mut self, action: Self::Actions) -> StateRewardPair {
        let transformed_action = NumCast::from(action).unwrap();
        let ActionReward { reward, done, .. } = self.environment.step(transformed_action);
        let reward_f32 = OrderedFloat(reward.into_inner() as f32);

        StateRewardPair {
            state: self.get_state(),
            reward: match done {
                true => Reward::Terminal(reward_f32),
                false => Reward::Terminal(reward_f32),
            },
        }
    }

    fn get_state(&self) -> Vec<RegisterValue> {
        let state = &self.environment.state;
        [state.position, state.velocity]
            .map(|v| OrderedFloat(v.into_inner() as f32))
            .to_vec()
    }

    fn finish(&mut self) {
        self.environment.close();
    }

    fn reset(&mut self) {
        self.environment.reset(None, false, None);
    }
}
