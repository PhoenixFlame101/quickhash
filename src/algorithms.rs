use md5;
use sha2::{Digest, Sha256};
use sha1::{Sha1, Digest};

pub fn calc_sha256(filepath: &str) -> String {
    let file_as_bytes = std::fs::read(filepath).unwrap();
    let hash = Sha256::digest(file_as_bytes);

    format!("{:x}", hash)
}

pub fn calc_md5(filepath: &str) -> String {
    let file_as_bytes = std::fs::read(filepath).unwrap();
    let hash = md5::compute(file_as_bytes);

    format!("{:x}", hash)
}
pub fn calc_sha1(filepath: &str) -> String {
    let file_as_bytes = std::fs::read(filepath).unwrap();
    let hash = Sha1::digest(file_as_bytes);
    
    format!("{:x}", hash)
}
