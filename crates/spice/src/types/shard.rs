use std::fmt::{Display, Formatter};
use std::ops::Add;
use borsh::BorshSerialize;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct ShardID(pub u32);

impl Add<u32> for ShardID {
    type Output = u32;

    fn add(self, rhs: u32) -> Self::Output {
        self.0 + rhs
    }
}

impl Display for ShardID {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
