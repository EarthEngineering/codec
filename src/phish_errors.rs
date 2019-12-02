use std::{error::Error, fmt};

/// Error concerning encoding/decoding of california addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PhishError {
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

impl fmt::Display for PhishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PhishError::ChecksumFailed(actual) => {
                write!(f, "invalid checksum (actual {} != 0)", actual)
            }
            PhishError::InvalidChar(index) => write!(f, "invalid char ({})", index),
            PhishError::NoPrefix => write!(f, "zero or multiple prefixes"),
            PhishError::MixedCase => write!(f, "mixed case string"),
            PhishError::InvalidVersion(c) => write!(f, "invalid version byte ({})", c),
            PhishError::InvalidPrefix(prefix) => write!(f, "invalid prefix ({})", prefix),
            PhishError::InvalidLength(length) => write!(f, "invalid length ({})", length),
        }
    }
}

impl Error for PhishError {
    fn cause(&self) -> Option<&dyn Error> {
        None
    }
    fn description(&self) -> &str {
        match *self {
            PhishError::ChecksumFailed { .. } => "invalid checksum",
            PhishError::InvalidChar(_) => "invalid char",
            PhishError::NoPrefix => "zero or multiple prefixes",
            PhishError::MixedCase => "mixed case string",
            PhishError::InvalidVersion(_) => "invalid version byte",
            PhishError::InvalidPrefix(_) => "invalid prefix",
            PhishError::InvalidLength(_) => "invalid length",
        }
    }
}
