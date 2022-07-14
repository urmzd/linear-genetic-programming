use std::fmt;

use ordered_float::OrderedFloat;

use crate::{
    core::registers::{MaybeBorrowed, RegisterValue},
    executable, executables,
};

#[derive(Clone)]
pub struct AnyExecutable(pub &'static str, pub InternalFn);

impl AnyExecutable {
    pub fn get_name(&self) -> &'static str {
        &self.0
    }

    pub fn get_fn(&self) -> InternalFn {
        self.1
    }
}

type InternalFn = for<'r, 's> fn(
    &'r mut [MaybeBorrowed<RegisterValue>],
    &'s [MaybeBorrowed<RegisterValue>],
) -> &'r [MaybeBorrowed<'r, RegisterValue>];

impl fmt::Debug for AnyExecutable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AnyExecutable").field(&self.0).finish()
    }
}

pub type Executables = &'static [AnyExecutable];

executable!(add, +);
executable!(subtract, -);
executable!(divide, /, OrderedFloat(2f32));
executable!(multiply, *);

pub const DEFAULT_EXECUTABLES: Executables = executables!(add, subtract, divide, multiply);
