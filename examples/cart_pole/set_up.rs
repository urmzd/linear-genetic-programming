use std::marker::PhantomData;

use derive_new::new;
use gym_rs::{core::Env, envs::classical_control::cartpole::CartPoleEnv};
use lgp::{
    core::{
        algorithm::GeneticAlgorithm, characteristics::Show, inputs::ValidInput, program::Program,
    },
    extensions::reinforcement_learning::{
        ReinforcementLearningInput, ReinforcementLearningParameters, Reward, StateRewardPair,
    },
    utils::executables::DEFAULT_EXECUTABLES,
};
use num::ToPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use ordered_float::OrderedFloat;
use serde::Serialize;
use strum::EnumCount;

#[derive(Debug, Clone, ToPrimitive, FromPrimitive, Serialize, PartialEq, Eq, EnumCount)]
pub enum CartPoleActions {
    Left = 0,
    Right = 1,
}

pub struct CartPoleLgp<'a>(PhantomData<&'a ()>);

#[derive(Clone, Debug, Serialize, new)]
pub struct CartPoleInput<'a> {
    environment: CartPoleEnv<'a>,
}

impl<'a> ValidInput for CartPoleInput<'a> {
    type Actions = CartPoleActions;

    const N_INPUTS: usize = 2;

    const AVAILABLE_EXECUTABLES: lgp::utils::executables::Executables = DEFAULT_EXECUTABLES;

    fn as_register_values(&self) -> Vec<lgp::core::registers::RegisterValue> {
        self.get_state()
    }
}

impl Show for CartPoleInput<'_> {}

impl<'a> ReinforcementLearningInput for CartPoleInput<'a> {
    fn init(&mut self) {
        self.environment.reset(None, false, None);
    }

    fn act(&mut self, action: Self::Actions) -> StateRewardPair {
        let discrete_action: usize =
            ToPrimitive::to_usize(&action).expect("Value to be derived from action.");
        let action_reward = self.environment.step(discrete_action);
        let reward = OrderedFloat(action_reward.reward.into_inner() as f32);

        StateRewardPair {
            state: self.get_state(),
            reward: match action_reward.done {
                true => Reward::Terminal(reward),
                false => Reward::Continue(reward),
            },
        }
    }

    fn reset(&mut self) {
        self.environment.reset(None, false, None);
    }

    fn get_state(&self) -> Vec<lgp::core::registers::RegisterValue> {
        let state = self.environment.state;
        let state_vec: Vec<_> = state.into();

        state_vec
            .iter()
            .map(move |s| OrderedFloat(*s as f32))
            .collect()
    }

    fn finish(&mut self) {
        self.environment.close()
    }
}

impl<'a> GeneticAlgorithm<'a> for CartPoleLgp<'a> {
    type O = Program<'a, ReinforcementLearningParameters<CartPoleInput<'a>>>;
}
