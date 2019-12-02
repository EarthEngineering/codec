use earth_codec::{Address, HashType, Network, Scheme};
use keyphrase::{KeyPhrase, KeyPhraseType, Language, Seed};

fn main() {
    better_panic::install();

    let scheme: Scheme = Scheme::Gabriel;
    let hash_type: HashType = HashType::Key;
    let network: Network = Network::Main;
    let keyphrase = KeyPhrase::new(KeyPhraseType::Words12, Language::English);
    let phrase: &str = keyphrase.phrase();
    println!("KeyPhrase: {}", phrase);

    // get the HD wallet seed
    let seed = Seed::new(&keyphrase, "");
    println!("Root Seed: {:X}", seed);
    println!("----------");

    // first encode an Address
    let body: Vec<u8> = seed.as_bytes().to_vec();

    let addr: Address = Address::new(body, scheme, hash_type, network);
    println!("{:#?}", addr);

    // Decode earth address
    // let earth_address: &str = "earth:er6m7j9njldwwzlg9v7v53unlr4jkmx6qy59n5mv54";

    // let mut earth_addr: Address = Address::decode(earth_address).unwrap();
    // println!("{:#?}", earth_addr);

    // Encode earth address
    let earth_address: String = addr.encode().unwrap();

    println!("{:#?}", earth_address);
    // earth:qp78r5zdgr53xszxlycksftf95wcv5a8q5m7zvw2aq
}
