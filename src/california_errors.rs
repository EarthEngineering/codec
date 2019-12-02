use std::{error::Error, fmt};

/// Error concerning encoding/decoding of california addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CaliforniaError {
    /// Invalid length (length)
    InvalidLength(usize),
    /// Zero or multiple prefixes
    NoPrefix,
    /// Failed to match known prefixes (prefix)
    InvalidPrefix(String),
    /// Checksum failed (checksum)
    ChecksumFailed(u64),
    /// Unexpected character (char)
    InvalidChar(char),
    /// Version byte was not recognized
    InvalidVersion(u8),
    /// Upper and lowercase address string
    MixedCase,
}

impl fmt::Display for CaliforniaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CaliforniaError::ChecksumFailed(actual) => {
                write!(f, "invalid checksum (actual {} != 0)", actual)
            }
            CaliforniaError::InvalidChar(index) => write!(f, "invalid char ({})", index),
            CaliforniaError::NoPrefix => write!(f, "zero or multiple prefixes"),
            CaliforniaError::MixedCase => write!(f, "mixed case string"),
            CaliforniaError::InvalidVersion(c) => write!(f, "invalid version byte ({})", c),
            CaliforniaError::InvalidPrefix(prefix) => write!(f, "invalid prefix ({})", prefix),
            CaliforniaError::InvalidLength(length) => write!(f, "invalid length ({})", length),
        }
    }
}

impl Error for CaliforniaError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            CaliforniaError::ChecksumFailed { .. } => "invalid checksum",
            CaliforniaError::InvalidChar(_) => "invalid char",
            CaliforniaError::NoPrefix => "zero or multiple prefixes",
            CaliforniaError::MixedCase => "mixed case string",
            CaliforniaError::InvalidVersion(_) => "invalid version byte",
            CaliforniaError::InvalidPrefix(_) => "invalid prefix",
            CaliforniaError::InvalidLength(_) => "invalid length",
        }
    }
}
