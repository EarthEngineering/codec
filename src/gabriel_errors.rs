use std::{error::Error, fmt};

/// Error concerning encoding/decoding of california addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GabrielError {
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

impl fmt::Display for GabrielError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GabrielError::ChecksumFailed(actual) => {
                write!(f, "invalid checksum (actual {} != 0)", actual)
            }
            GabrielError::InvalidChar(index) => write!(f, "invalid char ({})", index),
            GabrielError::NoPrefix => write!(f, "zero or multiple prefixes"),
            GabrielError::MixedCase => write!(f, "mixed case string"),
            GabrielError::InvalidVersion(c) => write!(f, "invalid version byte ({})", c),
            GabrielError::InvalidPrefix(prefix) => write!(f, "invalid prefix ({})", prefix),
            GabrielError::InvalidLength(length) => write!(f, "invalid length ({})", length),
        }
    }
}

impl Error for GabrielError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            GabrielError::ChecksumFailed { .. } => "invalid checksum",
            GabrielError::InvalidChar(_) => "invalid char",
            GabrielError::NoPrefix => "zero or multiple prefixes",
            GabrielError::MixedCase => "mixed case string",
            GabrielError::InvalidVersion(_) => "invalid version byte",
            GabrielError::InvalidPrefix(_) => "invalid prefix",
            GabrielError::InvalidLength(_) => "invalid length",
        }
    }
}
