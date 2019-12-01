use codec::{Address, Network, Scheme};

fn main() {
    better_panic::install();

    // Decode base58 address
    let legacy_addr: &str = "1CM18hbqJzCnM8CaxaNQHxJcnkcYbLV5Gw";

    // Decode cash address
    let cash_address: &str = "bitcoincash:qp78r5zdgr53xszxlycksftf95wcv5a8q5szslvspl";

    // Decode earth address
    // let earth_address: &str = "earth:qp78r5zdgr53xszxlycksftf95wcv5a8q5szslvspl";

    let mut addr: Address = Address::decode(legacy_addr).unwrap();
    println!("{:#?}", addr);

    let mut cash_addr: Address = Address::decode(cash_address).unwrap();
    println!("{:#?}", addr);

    // let mut earth_addr: Address = Address::decode(earth_address).unwrap();
    // println!("{:#?}", addr);

    // Change the base58 address to a test network earth address
    addr.network = Network::Main;
    addr.scheme = Scheme::CashAddr;

    // Encode cash address
    let cash_address: String = addr.encode().unwrap();

    println!("{:#?}", cash_address);
    // bitcoincash:qp78r5zdgr53xszxlycksftf95wcv5a8q5szslvspl
}
