use linked_hash_map::LinkedHashMap;
use std::{env, fs::File, io::prelude::*, io::Write, path::Path};
use toml;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    generate_regexes_mod(&out_dir);
    generate_fixture_tests_mod(&out_dir);
}

fn generate_regexes_mod(out_dir: &str) {
    let mut f = File::create(&Path::new(&out_dir).join("regexes.rs")).unwrap();

    f.write_all(
        b"
        #[macro_use]
        extern crate lazy_static;

        mod regexes {
            use regex::Regex;

            lazy_static! {
                pub static ref REGEXES: Vec<(Regex, Vec<&'static str>)> = vec![
        ",
    )
    .unwrap();

    let mut contents = Vec::new();
    File::open("data/regexes.toml")
        .unwrap()
        .read_to_end(&mut contents)
        .unwrap();

    let doc: LinkedHashMap<String, Vec<String>> = toml::from_slice(&contents.as_slice()).unwrap();

    for (k, v) in doc {
        f.write_all(
            &format!(
                "(Regex::new(r\"{}\").unwrap(), vec![\"{}\"]),\n",
                k,
                v.join("\", \""),
            )
            .into_bytes(),
        )
        .unwrap();
    }

    f.write_all(
        b"
                ];
            }
        }
        ",
    )
    .unwrap();
}

fn generate_fixture_tests_mod(out_dir: &str) {
    let mut f = File::create(&Path::new(&out_dir).join("fixture_tests.rs")).unwrap();

    f.write_all(
        b"
        #[cfg(test)]
        mod fixture_tests {
            use super::*;
        ",
    )
    .unwrap();

    let mut contents = Vec::new();
    File::open("data/fixtures.toml")
        .unwrap()
        .read_to_end(&mut contents)
        .unwrap();

    let doc: LinkedHashMap<String, Vec<String>> = toml::from_slice(&contents.as_slice()).unwrap();

    for (input, expected) in doc {
        let mut test_name = expected.clone().join("_");
        test_name.make_ascii_lowercase();
        test_name.truncate(64);
        test_name = test_name
            .chars()
            .map(|c| match c {
                'a'...'z' | '0'...'9' => c,
                _ => '_',
            })
            .collect();
        test_name = test_name.replace("__", "_");

        f.write_all(
            &format!(
                "
                #[test]
                fn test_{}() {{
                    assert_eq!(parse(&\"{}\"), vec![\"{}\"]);
                }}
                ",
                test_name,
                input,
                expected.join("\", \""),
            )
            .into_bytes(),
        )
        .unwrap();
    }

    f.write_all(b"}").unwrap();
}
