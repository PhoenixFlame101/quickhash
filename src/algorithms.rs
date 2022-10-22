use md5::Md5;
use sha2::{Sha256, Digest};
use sha1::Sha1;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn calc_sha256(filepath: &str) -> String {

    let file = File::open(filepath).unwrap();

    let reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    for l_result in reader.lines() {
        hasher.update(l_result.unwrap().as_bytes());
        hasher.update(b"\n");
    }
    let hash = hasher.finalize();

    format!("{:x}", hash)
}

pub fn calc_md5(filepath: &str) -> String {
    let file = File::open(filepath).unwrap();

    let reader = BufReader::new(file);
    let mut hasher = Md5::new();

    for l_result in reader.lines() {
        hasher.update(l_result.unwrap().as_bytes());
    }

    // acquire hash digest in the form of GenericArray,
    // which in this case is equivalent to [u8; 16]
    let result = hasher.finalize();
    format!("{:x}", result)
}

pub fn calc_sha1(filepath: &str) -> String {
    let file = File::open(filepath).unwrap();

    let reader = BufReader::new(file);
    let mut hasher = Sha1::new();
    for l_result in reader.lines() {
        hasher.update(l_result.unwrap().as_bytes());
    }
    let hash = hasher.finalize();

    format!("{:x}", hash)
}
