use std::marker::PhantomData;

use derive_new::new;

use crate::{
    core::{
        characteristics::Fitness,
        program::{ExtensionParameters, Program},
    },
    utils::common_traits::ValidInput,
};

#[derive(Debug, Clone, new)]
pub struct ReinforcementLearningParameters<T>
where
    T: ValidInput,
{
    marker: PhantomData<T>,
}

impl<T> ExtensionParameters for ReinforcementLearningParameters<T>
where
    T: ValidInput,
{
    type InputType = T;
}

pub trait ReinforcmentLearningInput: ValidInput {
    fn init_game(&self) -> ();
    fn act(&mut self, action: Self::Actions);
    fn finish(&mut self);
}

impl<'a, T> Fitness for Program<'a, ReinforcementLearningParameters<T>>
where
    T: ReinforcmentLearningInput,
{
    fn eval_fitness(&self) -> crate::core::characteristics::FitnessScore {
        todo!()
    }

    fn eval_set_fitness(&mut self) -> crate::core::characteristics::FitnessScore {
        todo!()
    }

    fn get_fitness(&self) -> Option<crate::core::characteristics::FitnessScore> {
        todo!()
    }
}
