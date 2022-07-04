use std::marker::PhantomData;

use derive_new::new;

use crate::{core::program::ExtensionParameters, utils::common_traits::ValidInput};

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
