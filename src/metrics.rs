use ordered_float::OrderedFloat;

use crate::fitness::FitnessScore;

pub trait Metric {
    type ObservableType;
    type ResultType;

    fn observe(&mut self, value: Self::ObservableType);
    fn calculate(&self) -> Self::ResultType;
}

// n_correct, total
pub struct Accuracy(usize, usize);

impl Accuracy {
    pub fn new(initial_correct: usize, total_counted: usize) -> Self {
        Accuracy(initial_correct, total_counted)
    }
}

impl Metric for Accuracy {
    type ObservableType = bool;
    type ResultType = FitnessScore;

    fn observe(&mut self, value: Self::ObservableType) {
        let count = match value {
            true => 1,
            _ => 0,
        };

        self.0 += count;
        self.1 += 1
    }

    fn calculate(&self) -> Self::ResultType {
        let Accuracy(n_correct, total) = self;
        OrderedFloat(*n_correct as f32) / OrderedFloat(*total as f32)
    }
}
