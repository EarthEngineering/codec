use crate::EarthError;
use std::{error::Error, fmt};

/// Error concerning encoding/decoding of addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AddressError {
    /// EarthAddr error
    Earth(EarthError),
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressError::Earth(ref e) => write!(f, "earthaddr error: {}", e),
        }
    }
}

impl Error for AddressError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            AddressError::Earth(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            AddressError::Earth(_) => "earthaddr error",
        }
    }
}
