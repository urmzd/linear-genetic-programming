use lgp::{
    core::registers::RegisterValue,
    executable, executables,
    utils::common_traits::{AnyExecutable, Executables},
};
use ordered_float::OrderedFloat;

executable!(add, +);
executable!(multiply, *);
executable!(subtract, -);
executable!(divide, /, OrderedFloat(2f32));

pub const IRIS_EXECUTABLES: Executables =
    executables!(self::add, self::subtract, self::divide, self::multiply);
