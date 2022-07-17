use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div},
};

use derive_new::new;
use gym_rs::core::ActionReward;
use gym_rs::{core::Env, envs::classical_control::mountain_car::MountainCarEnv};
use lgp::{
    core::{
        algorithm::GeneticAlgorithm,
        characteristics::{FitnessScore, Show},
        inputs::ValidInput,
        program::Program,
        registers::RegisterValue,
    },
    extensions::reinforcement_learning::{
        FitReward, ReinforcementLearningInput, ReinforcementLearningParameters, Reward,
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
    game: MountainCarEnv<'a>,
}

impl<'a> Show for MountainCarInput<'a> {}

impl<'a> ValidInput for MountainCarInput<'a> {
    type Actions = MountainCarActions;

    const N_INPUTS: usize = 2;

    const AVAILABLE_EXECUTABLES: Executables = DEFAULT_EXECUTABLES;

    fn as_register_values(&self) -> Vec<RegisterValue> {
        let state = self.get_state();
        state
    }
}

impl From<MountainCarRewardValue> for FitnessScore {
    fn from(reward: MountainCarRewardValue) -> Self {
        OrderedFloat(reward.0.into_inner() as f32)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd)]
pub struct MountainCarRewardValue(OrderedFloat<f64>);

impl Default for MountainCarRewardValue {
    fn default() -> Self {
        Self(OrderedFloat(0.))
    }
}

impl AddAssign for MountainCarRewardValue {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0)
    }
}

impl Add for MountainCarRewardValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Div<usize> for MountainCarRewardValue {
    type Output = FitnessScore;

    fn div(self, rhs: usize) -> Self::Output {
        OrderedFloat((self.0 / OrderedFloat(rhs as f64)).into_inner() as f32)
    }
}

impl FitReward for MountainCarRewardValue {}

impl<'a> ReinforcementLearningInput for MountainCarInput<'a> {
    type RewardValue = MountainCarRewardValue;

    fn init(&mut self) {
        self.game.reset(None, false, None);
    }

    fn act(
        &mut self,
        action: Self::Actions,
    ) -> lgp::extensions::reinforcement_learning::Reward<Self::RewardValue> {
        let transformed_action = NumCast::from(action).unwrap();
        let ActionReward { reward, done, .. } = self.game.step(transformed_action);
        if done {
            Reward::Terminal(MountainCarRewardValue(reward))
        } else {
            Reward::Continue(MountainCarRewardValue(reward))
        }
    }

    fn get_state(&self) -> Vec<RegisterValue> {
        let state = &self.game.state;
        [state.position, state.velocity]
            .map(|v| OrderedFloat(v.into_inner() as f32))
            .to_vec()
    }

    fn finish(&mut self) {
        self.game.close();
    }

    fn reset(&mut self) {
        self.game.reset(None, false, None);
    }
}
