use earth_codec::{Address, HashType, Network, Scheme};
use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};

fn main() {
    better_panic::install();

    let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::English);

    let seed = Seed::new(&keyphrase, "");

    // Set hash_type to Key for all examples
    let hash_type: HashType = HashType::Key;

    // Set network to Main for all examples
    let network: Network = Network::Regtest;

    // Set scheme to Gabriel
    let scheme: Scheme = Scheme::Earth;

    let body: Vec<u8> = seed.as_bytes().to_vec();

    let addr: Address = Address::new(body, scheme, hash_type, network);

    let gabriel_address: String = addr.encode().unwrap();
    println!("{:#?}", gabriel_address);
    // gabriel:guq9e8eamzc5ewf3tvcsdtqdx26qnp

    // Set scheme to California
    let scheme: Scheme = Scheme::California;

    let body: Vec<u8> = seed.as_bytes().to_vec();

    let addr: Address = Address::new(body, scheme, hash_type, network);

    let california_address: String = addr.encode().unwrap();
    println!("{:#?}", california_address);
    // california:cl7u0223yvhhslactcv8plvq0np
}
