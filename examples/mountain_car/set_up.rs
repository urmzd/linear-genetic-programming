use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div},
};

use derive_new::new;
use gym_rs::{core::ActionReward, utils::renderer::RenderMode};
use gym_rs::{core::Env, envs::classical_control::mountain_car::MountainCarEnv};
use lgp::{
    core::{
        algorithm::GeneticAlgorithm,
        characteristics::FitnessScore,
        instruction::{Mode, Modes},
        program::Program,
    },
    extensions::reinforcement_learning::{
        ReinforcementLearningInput, ReinforcementLearningParameters, Reward,
    },
    utils::common_traits::{Compare, Executables, Show, ValidInput, DEFAULT_EXECUTABLES},
};
use num::NumCast;
use num_derive::{FromPrimitive, ToPrimitive};
use ordered_float::OrderedFloat;
use serde::Serialize;
use strum::{Display, EnumCount};

#[derive(Debug, Clone, Display, Eq, PartialEq, EnumCount, FromPrimitive, ToPrimitive)]
pub enum Actions {
    AccelerateLeft = 0,
    Pause = 1,
    AccelerateRight = 2,
}

pub struct MountainCarLgp<'a>(PhantomData<&'a ()>);

impl<'a> GeneticAlgorithm<'a> for MountainCarLgp<'a> {
    type O = Program<'a, ReinforcementLearningParameters<MountainCarInput<'a>>>;
}

#[derive(Debug, Serialize, new)]
pub struct MountainCarInput<'a> {
    game: MountainCarEnv<'a>,
}

impl<'a> Clone for MountainCarInput<'a> {
    fn clone(&self) -> Self {
        Self {
            game: MountainCarEnv::new(RenderMode::None, None),
        }
    }
}

impl<'a> Ord for MountainCarInput<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }
}

impl<'a> PartialEq for MountainCarInput<'a> {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl<'a> Eq for MountainCarInput<'a> {}

impl<'a> PartialOrd for MountainCarInput<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl<'a> Show for MountainCarInput<'a> {}
impl<'a> Compare for MountainCarInput<'a> {}

impl<'a> ValidInput for MountainCarInput<'a> {
    type Actions = Actions;

    const AVAILABLE_EXECUTABLES: Executables = DEFAULT_EXECUTABLES;

    const AVAILABLE_MODES: Modes = Mode::INTERNAL_ONLY;

    fn generate_register_value_from(index: usize) -> lgp::core::registers::RegisterValue {
        if index < 3 {
            ordered_float::OrderedFloat(0f32)
        } else {
            ordered_float::OrderedFloat(1f32)
        }
    }
}

#[derive(Clone, Copy)]
pub struct MountainCarRewardValue(f64);

impl Default for MountainCarRewardValue {
    fn default() -> Self {
        Self(0.)
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
        OrderedFloat((self.0 / rhs as f64) as f32)
    }
}

impl<'a> ReinforcementLearningInput for MountainCarInput<'a> {
    type RewardValue = MountainCarRewardValue;

    fn init(&mut self) {
        self.game.reset();
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

    fn finish(&mut self) {
        // RENDER STUFF
    }
}
