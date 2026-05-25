use std::fmt;

#[derive(Debug)]
pub enum LavError {
    Exhausted,
    InvalidRange { min: u64, max: u64 },
    InvalidSeed(u64),
    CounterOverflow(u64),
}

impl fmt::Display for LavError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Exhausted => {
                write!(f, "ID pool exhausted, no more IDs available")
            }
            Self::InvalidRange { min, max } => {
                write!(
                    f,
                    "Invalid range: min({}) must be less than max({})",
                    min, max
                )
            }
            Self::InvalidSeed(n) => {
                write!(f, "Invalid seed: {} is out of range", n)
            }
            Self::CounterOverflow(n) => {
                write!(f, "Counter overflow at {}", n)
            }
        }
    }
}

impl std::error::Error for LavError {}
