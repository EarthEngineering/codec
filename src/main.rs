use codec::{Address, HashType, Network, Scheme};

fn main() {
    better_panic::install();

    // first encode an Address
    let body: Vec<u8> = vec![
        124, 113, 208, 77, 64, 233, 19, 64, 70, 249, 49, 104, 37, 105, 45, 29, 134, 83, 167, 5,
    ];

    let scheme: Scheme = Scheme::Earth;
    let hash_type: HashType = HashType::Script;
    let network: Network = Network::Main;
    let addr: Address = Address::new(body, scheme, hash_type, network);
    println!("{:#?}", addr);

    // Decode base58 address
    // let legacy_addr: &str = "1CM18hbqJzCnM8CaxaNQHxJcnkcYbLV5Gw";

    // Decode cash address
    // let cash_address: &str = "bitcoincash:qp78r5zdgr53xszxlycksftf95wcv5a8q5szslvspl";

    // Decode earth address
    // let earth_address: &str = "earth:qp78r5zdgr53xszxlycksftf95wcv5a8q5m7zvw2aq";

    // let mut addr: Address = Address::decode(legacy_addr).unwrap();
    // println!("{:#?}", addr);

    // let mut cash_addr: Address = Address::decode(cash_address).unwrap();
    // println!("{:#?}", addr);

    // let mut earth_addr: Address = Address::decode(earth_address).unwrap();
    // println!("{:#?}", earth_addr);

    // Change the base58 address to a test network earth address
    // earth_addr.network = Network::Main;
    // earth_addr.scheme = Scheme::Earth;

    // Encode cash address
    let earth_address: String = addr.encode().unwrap();

    println!("{:#?}", earth_address);
    // earth:qp78r5zdgr53xszxlycksftf95wcv5a8q5m7zvw2aq
}
