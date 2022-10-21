use md5::Context;
use sha2::{Digest as shaDigest, Sha256};
use std::io::{ prelude::*, BufReader};

pub fn calc_sha256(filepath: &str) -> String {
    // breaks file into lines and cumulatively calculates the hash
    let file = std::fs::File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut hasher = Sha256::new();
    for line in reader.lines(){
        match line {
            Ok(a) => {
                hasher.update(a.as_bytes());
                hasher.update(b"\n");
            },
            _ => break,
        };
    }
    let hash = hasher.finalize();

    format!("{:x}", hash)
}

pub fn calc_md5(filepath: &str) -> String {
    let file = std::fs::File::open(filepath).unwrap();
    let reader = BufReader::new(file);


    let mut hasher = Context::new();
    for line in reader.lines(){
        match line {
            Ok(a) => {
                hasher.consume(a.as_bytes());
                hasher.consume(b"\n");
            },
            _ => break,
        };
    }
    let hash = hasher.compute();
    format!("{:x}", hash)
}
