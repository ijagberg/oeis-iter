use std::fmt::Display;

pub mod fibonacci;
pub mod inventory;
pub mod look_and_see;
pub mod polygonal;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub struct Digit(u8);

impl Digit {
    pub fn new(v: u8) -> Result<Self, &'static str> {
        if v >= 10 {
            Err("invalid digit")
        } else {
            Ok(Self(v))
        }
    }
}

impl PartialOrd for Digit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Digit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}