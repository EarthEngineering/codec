use super::*;
use std::str::Chars;

// Prefixes
const MAINNET_PREFIX: &str = "california";
const TESTNET_PREFIX: &str = "californiatest";
const REGNET_PREFIX: &str = "californiareg";

// The california character set for encoding
const CHARSET: &[u8; 32] = b"cpzry9x8gf2tvdw0s3jn54khqe6mua7l";

// The california character set for decoding
#[rustfmt::skip]
const CHARSET_REV: [Option<u8>; 128] = [
    None,     None,     None,     None,     None,     None,     None,     None,
    None,     None,     None,     None,     None,     None,     None,     None,
    None,     None,     None,     None,     None,     None,     None,     None,
    None,     None,     None,     None,     None,     None,     None,     None,
    None,     None,     None,     None,     None,     None,     None,     None,
    None,     None,     None,     None,     None,     None,     None,     None,
    Some(15), None,     Some(10), Some(17), Some(21), Some(20), Some(26), Some(30),
    Some(7),  Some(5),  None,     None,     None,     None,     None,     None,
    None,     Some(29), None,     Some(24), Some(13), Some(25), Some(9),  Some(8),
    Some(23), None,     Some(18), Some(22), Some(31), Some(27), Some(19), None,
    Some(1),  Some(0),  Some(3),  Some(16), Some(11), Some(28), Some(12), Some(14),
    Some(6),  Some(4),  Some(2),  None,     None,     None,     None,     None,
    None,     Some(29),  None,    Some(24), Some(13), Some(25), Some(9),  Some(8),
    Some(23), None,     Some(18), Some(22), Some(31), Some(27), Some(19), None,
    Some(1),  Some(0),  Some(3),  Some(16), Some(11), Some(28), Some(12), Some(14),
    Some(6),  Some(4),  Some(2),  None,     None,     None,     None,     None,
];

// Version byte flags
#[allow(dead_code)]
mod version_byte_flags {
    pub const TYPE_MASK: u8 = 0x78;
    pub const TYPE_P2PKH: u8 = 0x00;
    pub const TYPE_P2SH: u8 = 0x08;
    pub const TYPE_ACCOUNT: u8 = 0x09;

    pub const SIZE_MASK: u8 = 0x07;
    pub const SIZE_160: u8 = 0x00;
    pub const SIZE_192: u8 = 0x01;
    pub const SIZE_224: u8 = 0x02;
    pub const SIZE_256: u8 = 0x03;
    pub const SIZE_320: u8 = 0x04;
    pub const SIZE_384: u8 = 0x05;
    pub const SIZE_448: u8 = 0x06;
    pub const SIZE_512: u8 = 0x07;
}

// https://github.com/Bitcoin-ABC/bitcoin-abc/blob/2804a49bfc0764ba02ce2999809c52b3b9bb501e/src/cashaddr.cpp#L42
fn polymod(v: &[u8]) -> u64 {
    let mut c: u64 = 1;
    for d in v.iter() {
        let c0: u8 = (c >> 35) as u8;
        c = ((c & 0x0007_ffff_ffff) << 5) ^ u64::from(*d);
        if c0 & 0x01 != 0 {
            c ^= 0x0098_f2bc_8e61;
        }
        if c0 & 0x02 != 0 {
            c ^= 0x0079_b76d_99e2;
        }
        if c0 & 0x04 != 0 {
            c ^= 0x00f3_3e5f_b3c4;
        }
        if c0 & 0x08 != 0 {
            c ^= 0x00ae_2eab_e2a8;
        }
        if c0 & 0x10 != 0 {
            c ^= 0x001e_4f43_e470;
        }
    }
    c ^ 1
}

// Expand the address prefix for the checksum operation
fn expand_prefix(prefix: &str) -> Vec<u8> {
    let mut ret: Vec<u8> = prefix.chars().map(|c| (c as u8) & 0x1f).collect();
    ret.push(0);
    ret
}

fn convert_bits(data: &[u8], inbits: u8, outbits: u8, pad: bool) -> Vec<u8> {
    assert!(inbits <= 8 && outbits <= 8);
    let num_bytes = (data.len() * inbits as usize + outbits as usize - 1) / outbits as usize;
    let mut ret = Vec::with_capacity(num_bytes);
    let mut acc: u16 = 0; // accumulator of bits
    let mut num: u8 = 0; // num bits in acc
    let groupmask = (1 << outbits) - 1;
    for d in data.iter() {
        // We push each input chunk into a 16-bit accumulator
        acc = (acc << inbits) | u16::from(*d);
        num += inbits;
        // Then we extract all the output groups we can
        while num > outbits {
            ret.push((acc >> (num - outbits)) as u8);
            acc &= !(groupmask << (num - outbits));
            num -= outbits;
        }
    }
    if pad {
        // If there's some bits left, pad and add it
        if num > 0 {
            ret.push((acc << (outbits - num)) as u8);
        }
    } else {
        // If there's some bits left, figure out if we need to remove padding and add it
        let padding = (data.len() * inbits as usize) % outbits as usize;
        if num as usize > padding {
            ret.push((acc >> padding) as u8);
        }
    }
    ret
}

/// Codec allowing the encoding and decoding of california addresses
pub struct CaliforniaCodec;

impl AddressCodec for CaliforniaCodec {
    type Error = CaliforniaError;
    fn encode(raw: &[u8], hash_type: HashType, network: Network) -> Result<String, Self::Error> {
        // Calculate version byte
        let hash_flag: u8 = match hash_type {
            HashType::Key => version_byte_flags::TYPE_P2PKH,
            HashType::Script => version_byte_flags::TYPE_P2SH,
            HashType::Account => version_byte_flags::TYPE_P2PKH,
        };
        let length: usize = raw.len();
        let version_byte: u8 = match length {
            20 => version_byte_flags::SIZE_160,
            24 => version_byte_flags::SIZE_192,
            28 => version_byte_flags::SIZE_224,
            32 => version_byte_flags::SIZE_256,
            40 => version_byte_flags::SIZE_320,
            48 => version_byte_flags::SIZE_384,
            56 => version_byte_flags::SIZE_448,
            64 => version_byte_flags::SIZE_512,
            _ => return Err(CaliforniaError::InvalidLength(length).into()),
        } | hash_flag;

        // Get prefix
        let prefix: &str = match network {
            Network::Main => MAINNET_PREFIX,
            Network::Test => TESTNET_PREFIX,
            Network::Regtest => REGNET_PREFIX,
        };

        // Convert payload to 5 bit array
        let mut payload: Vec<u8> = Vec::with_capacity(1 + raw.len());
        payload.push(version_byte);
        payload.extend(raw);
        let payload_5_bits: Vec<u8> = convert_bits(&payload, 8, 5, true);

        // Construct payload string using CHARSET
        let payload_str: String = payload_5_bits
            .iter()
            .map(|b| CHARSET[*b as usize] as char)
            .collect();

        // Create checksum
        let expanded_prefix: Vec<u8> = expand_prefix(prefix);
        let checksum_input: Vec<u8> = [&expanded_prefix[..], &payload_5_bits, &[0; 8][..]].concat();
        let checksum: u64 = polymod(&checksum_input);

        // Convert checksum to string
        let checksum_str: String = (0..8)
            .rev()
            .map(|i| CHARSET[((checksum >> (i * 5)) & 31) as usize] as char)
            .collect();

        // Concatentate all parts
        let earthaddr: String = [prefix, ":", &payload_str, &checksum_str].concat();
        Ok(earthaddr)
    }

    fn decode(addr_str: &str) -> Result<Address, Self::Error> {
        // Delimit and extract prefix
        let parts: Vec<&str> = addr_str.split(':').collect();
        if parts.len() != 2 {
            return Err(CaliforniaError::NoPrefix.into());
        }
        let prefix: &str = parts[0];
        let payload_str: &str = parts[1];

        // Match network
        let network: Network = match prefix {
            MAINNET_PREFIX => Network::Main,
            TESTNET_PREFIX => Network::Test,
            REGNET_PREFIX => Network::Regtest,
            _ => return Err(CaliforniaError::InvalidPrefix(prefix.to_string()).into()),
        };

        // Do some sanity checks on the string
        let mut payload_chars = payload_str.chars();
        if let Some(first_char) = payload_chars.next() {
            if first_char.is_lowercase() {
                if payload_chars.any(|c| c.is_uppercase()) {
                    return Err(CaliforniaError::MixedCase.into());
                }
            } else if payload_chars.any(|c| c.is_lowercase()) {
                return Err(CaliforniaError::MixedCase.into());
            }
        } else {
            return Err(CaliforniaError::InvalidLength(0).into());
        }

        // Decode payload to 5 bit array
        let payload_chars: Chars<'_> = payload_str.chars(); // Reintialize iterator here
        let payload_5_bits: Result<Vec<u8>, CaliforniaError> = payload_chars
            .map(|c| {
                let i = c as usize;
                if let Some(Some(d)) = CHARSET_REV.get(i) {
                    Ok(*d as u8)
                } else {
                    Err(CaliforniaError::InvalidChar(c))
                }
            })
            .collect();
        let payload_5_bits: Vec<u8> = payload_5_bits?;

        // Verify the checksum
        let checksum: u64 = polymod(&[&expand_prefix(prefix), &payload_5_bits[..]].concat());
        if checksum != 0 {
            return Err(CaliforniaError::ChecksumFailed(checksum).into());
        }

        // Convert from 5 bit array to byte array
        let len_5_bit: usize = payload_5_bits.len();
        let payload: Vec<u8> = convert_bits(&payload_5_bits[..(len_5_bit - 8)], 5, 8, false);

        // Verify the version byte
        let version: u8 = payload[0];

        // Check length
        let body: &[u8] = &payload[1..];
        let body_len: usize = body.len();
        let version_size: u8 = version & version_byte_flags::SIZE_MASK;
        if (version_size == version_byte_flags::SIZE_160 && body_len != 20)
            || (version_size == version_byte_flags::SIZE_192 && body_len != 24)
            || (version_size == version_byte_flags::SIZE_224 && body_len != 28)
            || (version_size == version_byte_flags::SIZE_256 && body_len != 32)
            || (version_size == version_byte_flags::SIZE_320 && body_len != 40)
            || (version_size == version_byte_flags::SIZE_384 && body_len != 48)
            || (version_size == version_byte_flags::SIZE_448 && body_len != 56)
            || (version_size == version_byte_flags::SIZE_512 && body_len != 64)
        {
            return Err(CaliforniaError::InvalidLength(body_len).into());
        }

        // Extract the hash type and return
        let version_type: u8 = version & version_byte_flags::TYPE_MASK;
        let hash_type: HashType = if version_type == version_byte_flags::TYPE_P2PKH {
            HashType::Key
        } else if version_type == version_byte_flags::TYPE_P2SH {
            HashType::Script
        } else {
            return Err(CaliforniaError::InvalidVersion(version).into());
        };

        Ok(Address {
            scheme: Scheme::California,
            body: body.to_vec(),
            hash_type,
            network,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex;

    // TODO - FIX THESE TESTS
    // #[test]
    // fn mainnet_20byte() {
    //     // 20-byte public key hash on testnet
    //     verify(
    //         Network::Test,
    //         &hex::decode("C062A7AAD22E63177192C880D7D248CA6F2B9A6525F69A7CDE65561326888BD97A519E65890886B265C34CECDFC2F34237D487E71FC81C1A44C88E872EB993").unwrap(),
    //         "earth:elex9fa26ghxx9m3jtygp4cdyjx2du4q5qf976d8qhn92cfjdzytm9a9r8n93yygdvn9cdxwqh7z7dpr04y8uug0qewp53xg36rjawvnaeep36h9",
    //     );
}

// #[test]
// fn mainnet_24byte() {
//     // 24-byte public key hash on mainnet
//     verify(
//         Network::Main,
//         &hex::decode("7ADBF6C17084BC86C1706827B41A56F5CA32865925E946EA").unwrap(),
//         "earth:elex9fa26ghxx9m3jtygp4cdyjx2du4q5qf976d8qhn92cfjdzytm9a9r8n93yygdvn9cdxwqh7z7dpr04y8uug0qewp53xg36rjawvnaeep36h9",
//     );
// }

// #[test]
// fn mainnet_28byte() {
//     // 28-byte public key hash on mainnet
//     verify(
//         Network::Main,
//         &hex::decode("3A84F9CF51AAE98A3BB3A78BF16A6183790B18719126325BFC0C075B").unwrap(),
//         "earth:elex9fa26ghxx9m3jtygp4cdyjx2du4q5qf976d8qhn92cfjdzytm9a9r8n93yygdvn9cdxwqh7z7dpr04y8uug0qewp53xg36rjawvnaeep36h9",
//     );
// }

// #[test]
// fn mainnet_32byte() {
//     // 32-byte public key hash on mainnet
//     verify(
//         Network::Main,
//         &hex::decode("3173EF6623C6B48FFD1A3DCC0CC6489B0A07BB47A37F47CFEF4FE69DE825C060")
//             .unwrap(),
//         "earth:elex9fa26ghxx9m3jtygp4cdyjx2du4q5qf976d8qhn92cfjdzytm9a9r8n93yygdvn9cdxwqh7z7dpr04y8uug0qewp53xg36rjawvnaeep36h9",
//     );
// }

// #[test]
// fn mainnet_40byte() {
//     // 40-byte public key hash on mainnet
//     verify(
//         Network::Main,
//         &hex::decode(
//             "C07138323E00FA4FC122D3B85B9628EA810B3F381706385E289B0B25631197D194B5C238BEB136FB",
//         )
//         .unwrap(),
//         "earth:elex9fa26ghxx9m3jtygp4cdyjx2du4q5qf976d8qhn92cfjdzytm9a9r8n93yygdvn9cdxwqh7z7dpr04y8uug0qewp53xg36rjawvnaeep36h9",
//     );
// }

// #[test]
// fn mainnet_48byte() {
//     // 48-byte public key hash on mainnet
//     verify(
//         Network::Main,
//         &hex::decode("E361CA9A7F99107C17A622E047E3745D3E19CF804ED63C5C40C6BA763696B98241223D8CE62AD48D863F4CB18C930E4C").unwrap(),
//         "earth:elex9fa26ghxx9m3jtygp4cdyjx2du4q5qf976d8qhn92cfjdzytm9a9r8n93yygdvn9cdxwqh7z7dpr04y8uug0qewp53xg36rjawvnaeep36h9",
//     );
// }

// #[test]
// fn mainnet_56byte() {
//     // 56-byte public key hash on mainnet
//     verify(
//         Network::Main,
//         &hex::decode("D9FA7C4C6EF56DC4FF423BAAE6D495DBFF663D034A72D1DC7D52CBFE7D1E6858F9D523AC0A7A5C34077638E4DD1A701BD017842789982041").unwrap(),
//         "earth:emvl5lzvdm6km38lgga64qk5jhdl7q3aed9895wu04fvhlnarq5937w4ywke57juxsrhvw8ym5d8ex7sz7zz0zvcypesuhtawhge",
//     );
// }
// #[test]
// fn mainnet_64byte() {
//     // 64-byte public key hash on mainnet
//     verify(
//         Network::Main,
//         &hex::decode("D0F346310D5513D9E01E299978624BA883E6BDA8F4C60883C10F28C2967E67EC77ECC7EEEAEAFC6DA89FAD72D11AC961E164678B868AEEEC5F2C1DA08884175B").unwrap(),
//         "earth:elg0x333p4238k0erc5qj7rzfw5g8q4a4r6vvzyrcy8j3s5k0qn7calvclhw46hudk5flttj6ydvjc0pv3nchp52amk97tea5zygg96mpztzj0dc",
//     );
// }

fn verify(network: Network, data: &Vec<u8>, earthaddr: &str) {
    let hash_type = HashType::Key;
    let output = EarthCodec::encode(data, hash_type, network).unwrap();
    println!("{:#?}", output);
    assert!(output == earthaddr.to_ascii_lowercase());
    let decoded = EarthCodec::decode(earthaddr).unwrap();
    assert!(decoded.as_ref().to_vec() == *data);
}
// }
