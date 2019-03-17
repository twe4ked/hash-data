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

include!(concat!(env!("OUT_DIR"), "/regexes.rs"));

use regexes::REGEXES;

/// Parses the hash and returns potential hash types.
///
/// ```
/// assert_eq!(hash_data::parse("$1$42bad211$ums.eDtzK/1711rUkRsd31"), vec!["MD5(Unix)"])
/// ```
pub fn parse(input: &str) -> Vec<&str> {
    if input.is_empty() {
        return Vec::new();
    }

    if let Some((_r, v)) = REGEXES.iter().find(|&(r, _v)| r.is_match(&input)) {
        v.clone()
    } else {
        Vec::new()
    }
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
