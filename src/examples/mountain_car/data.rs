use std::marker::PhantomData;

use num_derive::FromPrimitive;
use strum::{Display, EnumCount};

use crate::genes::algorithm::GeneticAlgorithm;

#[derive(Debug, Clone, Display, Eq, PartialEq, EnumCount, FromPrimitive)]
pub enum Actions {
    AccelerateLeft = 0,
    AccelerateRight = 1,
    Pause = 2,
}

struct MountainCarLgp<'a>(PhantomData<&'a ()>);

impl<'a> GeneticAlgorithm<'a> for MountainCarLgp<'a> {
}


