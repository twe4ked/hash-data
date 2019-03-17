#![deny(missing_docs)]

//! A library and command line tool for identifying hashes.
//!
//! # Examples
//!
//! Using the library:
//!
//! ```
//! assert_eq!(hash_data::parse("$1$42bad211$ums.eDtzK/1711rUkRsd31"), vec!["MD5(Unix)"])
//! ```
//!
//! On the command line:
//!
//! ```sh
//! $ hash-data '$1$42bad211$ums.eDtzK/1711rUkRsd31'
//! MD5(Unix)
//! ```
//!
//! # Supported hash types:
//!
//! - Adler32
//! - Base64
//! - Blowfish
//!     - Eggdrop
//!     - OpenBSD
//! - CRC
//!     - CRC-16, CRC-16-CCITT
//!     - CRC-32
//!     - CRC-32B
//!     - CRC-96(ZIP)
//! - DES
//!     - Oracle
//!     - Unix
//! - Domain Cached Credentials
//!     - Domain Cached Credentials 2
//! - FCS
//!     - FCS-16
//!     - FCS-32
//! - FNV
//!     - FNV-132
//!     - FNV-164
//! - GHash
//!     - GHash-32-3
//!     - GHash-32-5
//! - GOST R 34.11-94
//! - Haval
//!     - Haval-128
//!     - Haval-160
//!     - Haval-192
//!     - Haval-224
//!     - Haval-256
//! - Joaat
//! - Keccak
//!     - Keccak-224
//!     - Keccak-256
//! - LM
//! - Lineage II C4
//! - Lotus Domino
//! - MD2
//! - MD4
//! - MD5
//!     - APR
//!     - Cisco PIX
//!     - IP.Board
//!     - Joomla
//!     - MyBB
//!     - Palshop
//!     - Unix
//!     - Wordpress
//!     - osCommerce
//!     - phpBB3
//! - MSSQL
//!     - MSSQL(2000)
//!     - MSSQL(2005)
//!     - MSSQL(2008)
//! - MySQL
//!     - MySQL3.x
//!     - MySQL4.x
//!     - MySQL5.x
//! - NTLM
//! - RAdminv2.x
//! - RIPEMD
//!     - RIPEMD-128
//!     - RIPEMD-160
//!     - RIPEMD-256
//!     - RIPEMD-320
//! - SAM(LM_Hash:NT_Hash)
//! - SHA
//!     - SHA-1(Django)
//!     - SHA-1(MaNGOS)
//!     - SHA-1(MaNGOS2)
//!     - SHA-224
//!     - SHA-256
//!     - SHA-256(Django)
//!     - SHA-256(Unix)
//!     - SHA-384
//!     - SHA-384(Django)
//!     - SHA-512
//!     - SHA-512(Drupal)
//!     - SHA-512(Unix)
//!     - SHA3-384
//!     - SHA3-512
//! - SSHA-1
//! - Skein
//!     - Skein-256(128, 160, 224)
//!     - Skein-512(128, 160, 224, 256, 384)
//!     - Skein-1024(384, 512)
//! - Snefru
//!   - Snefru-128
//!   - Snefru-256
//! - Tiger
//!   - Tiger-128
//!   - Tiger-160
//!   - Tiger-192
//! - VNC
//! - Whirlpool
//! - XOR-32

include!(concat!(env!("OUT_DIR"), "/regexes.rs"));

use regexes::REGEXES;

/// Parses the hash and returns potential hash types.
///
/// ```
/// assert_eq!(hash_data::parse("$1$42bad211$ums.eDtzK/1711rUkRsd31"), vec!["MD5(Unix)"])
/// ```
pub fn parse(input: &str) -> Vec<&str> {
    let matches = Vec::new();

    if input.is_empty() {
        return matches;
    }

    REGEXES.iter().fold(matches, |mut a, (r, v)| {
        if r.is_match(&input) {
            a.append(&mut v.clone());
        };
        a
    })
}

include!(concat!(env!("OUT_DIR"), "/fixture_tests.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        assert_eq!(parse(&""), vec![] as Vec<&str>)
    }

    #[test]
    fn test_no_match() {
        assert_eq!(parse(&"not a matching hash"), vec![] as Vec<&str>)
    }
}
