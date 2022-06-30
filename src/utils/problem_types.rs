use super::common_traits::ValidInput;

pub trait ClassificationProblem: ValidInput {
    fn get_class(&self) -> Self::Represent;
}
