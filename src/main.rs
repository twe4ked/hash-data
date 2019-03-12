use hash_data;
use std::{env, path::Path, process};

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 2 {
        let matches = hash_data::parse(&args[1]);
        if matches.is_empty() {
            eprintln!("No match");
            process::exit(1);
        } else {
            println!("{}", matches.join(", "));
            process::exit(0);
        }
    } else {
        let mut name = None;
        if let Some(file_name) = Path::new(&args[0]).file_name() {
            name = file_name.to_str()
        };
        eprintln!("usage: {} <hash>", name.unwrap_or("hash-data"));
        process::exit(1);
    }
}
