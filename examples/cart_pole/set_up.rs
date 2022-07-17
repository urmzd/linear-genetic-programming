use num_derive::{FromPrimitive, ToPrimitive};
use serde::Serialize;

#[derive(Debug, Clone, ToPrimitive, FromPrimitive, Serialize, PartialEq, Eq)]
pub enum CartPoleActions {
    Left = 0,
    Right = 1,
}

pub struct CartPoleLgp<'a>(PhantomData<&'a ()>);
