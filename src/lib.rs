//! Codec
//!
//! A simple library providing an `Address` struct enabling
//! encoding/decoding of EARTH addresses.
//!
//! ```rust
//! use earth_codec::{Address, Network, Scheme};
//!
//! fn main() {
//! // Decode earth address
//! let earth_addr: &str = "earthtest:qp78r5zdgr53xszxlycksftf95wcv5a8q5khw5038k";
//! let mut addr = Address::decode(earth_addr).unwrap();
//!
//! // Change the base58 address to a test network earth address
//! addr.network = Network::Test;
//! addr.scheme = Scheme::Earth;
//!
//! // Encode earth address
//! let earth_address: String = addr.encode().unwrap();
//!
//! println!("{:#?}", earth_address);
//! // earthtest:qp78r5zdgr53xszxlycksftf95wcv5a8q5khw5038k
//! }
//! ```
//!

mod california;
mod california_errors;
mod earth;
mod earth_errors;
mod errors;
mod gabriel;
mod gabriel_errors;
mod phish;
mod phish_errors;

pub use california::CaliforniaCodec;
pub use california_errors::CaliforniaError;
pub use earth::EarthCodec;
pub use earth_errors::EarthError;
pub use errors::*;
pub use gabriel::GabrielCodec;
pub use gabriel_errors::GabrielError;
pub use phish::PhishCodec;
pub use phish_errors::PhishError;

/// EARTH Networks
#[derive(PartialEq, Clone, Debug)]
pub enum Network {
    /// Main network
    Main,
    /// Test network
    Test,
    /// Regression test network
    Regtest,
}

/// Address encoding scheme
#[derive(PartialEq, Clone, Debug)]
pub enum Scheme {
    /// Earth Address encoding
    Earth,
    /// California Address encoding
    California,
    /// Phish Address encoding
    Phish,
    /// Gabriel Address encoding
    Gabriel,
}

/// Intepretation of the Hash160 bytes
#[derive(PartialEq, Clone, Debug)]
pub enum HashType {
    /// Public key hash
    Key,
    /// Script hash
    Script,
    /// Account,
    Account,
}

/// Struct containing the bytes and metadata of a Bitcoin Cash address.
/// This is yeilded during decoding or consumed during encoding.
#[derive(PartialEq, Clone, Debug)]
pub struct Address {
    /// Address bytes
    pub body: Vec<u8>,
    /// Encoding scheme
    pub scheme: Scheme,
    /// Hash type
    pub hash_type: HashType,
    /// Network
    pub network: Network,
}

/// Creates an empty `Address` struct, with the `body` bytes the empty vector,
/// `Scheme::Earth`, `HashType::Key`, and `Network::Main`.
impl Default for Address {
    fn default() -> Self {
        Address {
            body: vec![],
            scheme: Scheme::Earth,
            hash_type: HashType::Key,
            network: Network::Main,
        }
    }
}

impl<'a> AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.body
    }
}

impl Address {
    /// Create a new address
    pub fn new(body: Vec<u8>, scheme: Scheme, hash_type: HashType, network: Network) -> Self {
        Address {
            body,
            scheme,
            hash_type,
            network,
        }
    }

    /// Take address bytes
    pub fn into_body(self) -> Vec<u8> {
        self.body
    }

    /// Attempt to convert the raw address bytes to a string
    pub fn encode(&self) -> Result<String, AddressError> {
        match self.scheme {
            Scheme::Earth => EarthCodec::encode(
                &self.body,
                self.hash_type.to_owned(),
                self.network.to_owned(),
            )
            .map_err(AddressError::Earth),
            Scheme::California => CaliforniaCodec::encode(
                &self.body,
                self.hash_type.to_owned(),
                self.network.to_owned(),
            )
            .map_err(AddressError::California),
            Scheme::Phish => PhishCodec::encode(
                &self.body,
                self.hash_type.to_owned(),
                self.network.to_owned(),
            )
            .map_err(AddressError::Phish),
            Scheme::Gabriel => GabrielCodec::encode(
                &self.body,
                self.hash_type.to_owned(),
                self.network.to_owned(),
            )
            .map_err(AddressError::Gabriel),
        }
    }

    /// Attempt to convert an address string into bytes
    pub fn decode(addr_str: &str) -> Result<Self, (EarthError)> {
        EarthCodec::decode(addr_str).map_err(|earth_err| (earth_err))
    }
}

/// A trait providing an interface for encoding and decoding the `Address` struct
/// for each address scheme.
pub trait AddressCodec {
    type Error;
    /// Attempt to convert the raw address bytes to a string
    fn encode(raw: &[u8], hash_type: HashType, network: Network) -> Result<String, Self::Error>;

    /// Attempt to convert the address string to bytes
    fn decode(s: &str) -> Result<Address, Self::Error>;
}
