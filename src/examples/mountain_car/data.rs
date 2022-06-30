use strum::{Display, EnumCount};

#[derive(Debug, Clone, Display, Eq, PartialEq, EnumCount)]
pub enum Actions {
    AccelerateLeft = 0,
    AccelerateRight = 1,
    Pause = 2,
}
