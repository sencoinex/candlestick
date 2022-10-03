use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    InvalidInterval {
        value: String,
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidInterval { value } => {
                write!(f, "Illegal interval string: {}", value)
            }
        }
    }
}
