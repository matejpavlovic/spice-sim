use std::cmp::Ordering;
use std::ops::Mul;
use borsh::BorshSerialize;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, BorshSerialize)]
pub struct Height(pub u32);

impl Ord for Height {
    fn cmp(&self, other: &Height) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Height {
    fn partial_cmp(&self, other: &Height) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::ops::Add for Height {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl From<Height> for u32 {
    fn from(value: Height) -> Self {
        value.0
    }
}

impl Mul<u32> for Height {
    type Output = u32;

    fn mul(self, rhs: u32) -> Self::Output {
        self.0 * rhs
    }
}
