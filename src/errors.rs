use crate::{CaliforniaError, EarthError, GabrielError, PhishError};
use std::{error::Error, fmt};

/// Error concerning encoding/decoding of addresses
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AddressError {
    /// Earth Address error
    Earth(EarthError),
    /// California Address error
    California(CaliforniaError),
    /// Phish Address error
    Phish(PhishError),
    /// Gabriel Address error
    Gabriel(GabrielError),
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddressError::Earth(ref e) => write!(f, "earth address error: {}", e),
            AddressError::California(ref e) => write!(f, "california address error: {}", e),
            AddressError::Phish(ref e) => write!(f, "phish address error: {}", e),
            AddressError::Gabriel(ref e) => write!(f, "gabriel address error: {}", e),
        }
    }
}

impl Error for AddressError {
    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            AddressError::Earth(ref e) => Some(e),
            AddressError::California(ref e) => Some(e),
            AddressError::Phish(ref e) => Some(e),
            AddressError::Gabriel(ref e) => Some(e),
        }
    }

    fn description(&self) -> &str {
        match *self {
            AddressError::Earth(_) => "earth address error",
            AddressError::California(_) => "california address error",
            AddressError::Phish(_) => "phish address error",
            AddressError::Gabriel(_) => "gabriel address error",
        }
    }
}
