use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use borsh::BorshSerialize;

#[derive(Copy, Clone, Eq, PartialEq, Hash, BorshSerialize)]
pub struct Hash(pub(crate) [u8; 32]);

impl Display for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}{:02x}", self.0[0], self.0[1], self.0[2], self.0[3])
    }
}

impl Debug for Hash {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hash({})", self)
    }
}

impl Ord for Hash {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Hash {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
