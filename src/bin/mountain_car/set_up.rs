use std::{
    marker::PhantomData,
    ops::{Add, AddAssign, Div},
};

use gym_rs::MountainCarEnv;
use lgp::{
    core::{
        characteristics::FitnessScore,
        instruction::{Mode, Modes},
    },
    extensions::reinforcement_learning::ReinforcementLearningInput,
    utils::common_traits::{Compare, Executables, Show, ValidInput, DEFAULT_EXECUTABLES},
};
use num::bigint::ParseBigIntError;
use num_derive::FromPrimitive;
use ordered_float::OrderedFloat;
use serde::Serialize;
use strum::{Display, EnumCount};

#[derive(Debug, Clone, Display, Eq, PartialEq, EnumCount, FromPrimitive)]
enum Actions {
    AccelerateLeft = 0,
    AccelerateRight = 1,
    Pause = 2,
}

struct MountainCarLgp<'a>(PhantomData<&'a ()>);

#[derive(Debug, Serialize)]
struct MountainCarInput {
    game: MountainCarEnv,
}

impl Clone for MountainCarInput {
    fn clone(&self) -> Self {
        Self {
            game: MountainCarEnv::default(),
        }
    }
}

impl Ord for MountainCarInput {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmp(other)
    }
}

impl PartialEq for MountainCarInput {
    fn eq(&self, other: &Self) -> bool {
        true
    }
}

impl Eq for MountainCarInput {}

impl PartialOrd for MountainCarInput {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
    }
}

impl Show for MountainCarInput {}
impl Compare for MountainCarInput {}

impl ValidInput for MountainCarInput {
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
struct MountainCarRewardValue(usize);

impl Default for MountainCarRewardValue {
    fn default() -> Self {
        Self(0)
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
        OrderedFloat((self.0 / rhs) as f32)
    }
}

impl ReinforcementLearningInput for MountainCarInput {
    type RewardValue = MountainCarRewardValue;
}

// impl<'a> GeneticAlgorithm<'a> for MountainCarLgp<'a> {
//     type O;
// }
