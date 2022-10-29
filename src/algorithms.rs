use md5;
use sha1::Sha1;
use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader};

pub fn calc_sha256(filepath: &str) -> String {
    //println!("{}", filepath);
    const CAP: usize = 10*1024*1024; 
    let file = match std::fs::File::open(filepath) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut hasher = Sha256::new();
    let mut reader = BufReader::with_capacity(CAP, file);
    loop{
        let length = {
            let buffer = match reader.fill_buf() {
                Ok(buffer) => buffer,
                Err(_) => panic!("Error reading Data"),
            };
            hasher.update(buffer);
            buffer.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length)
    }
    //hasher.update(file_as_bytes);
    
    let hash = hasher.finalize();


    //let hash = Sha256::digest(file_as_bytes);
    format!("{:x}", hash)
}

pub fn calc_md5(filepath: &str) -> String {
    // let file_as_bytes = std::fs::read(filepath).unwrap();
    // let hash = md5::compute(file_as_bytes);

    const CAP: usize = 10*1024*1024; 
    let file = match std::fs::File::open(filepath) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut hasher = md5::Context::new();
    let mut reader = BufReader::with_capacity(CAP, file);
    loop{
        let length = {
            let buffer = match reader.fill_buf() {
                Ok(buffer) => buffer,
                Err(_) => panic!("Error reading Data"),
            };
            hasher.consume(buffer);
            buffer.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length)
    }

    let hash = hasher.compute();

    format!("{:x}", hash)
}

pub fn calc_sha1(filepath: &str) -> String {
    // let file_as_bytes = std::fs::read(filepath).unwrap();
    // let hash = Sha1::digest(file_as_bytes);

    const CAP: usize = 10*1024*1024; 
    let file = match std::fs::File::open(filepath) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };
    let mut hasher = Sha1::new();
    let mut reader = BufReader::with_capacity(CAP, file);
    loop{
        let length = {
            let buffer = match reader.fill_buf() {
                Ok(buffer) => buffer,
                Err(_) => panic!("Error reading Data"),
            };
            hasher.update(buffer);
            buffer.len()
        };
        if length == 0 {
            break;
        }
        reader.consume(length)
    }

    let hash = hasher.finalize();

    format!("{:x}", hash)
}
