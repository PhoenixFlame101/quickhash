use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::fs::File;
use std::io::prelude::*;
use std::time::{Instant};
use std::process::exit;

mod algorithms;

fn lines_from_file(filename: &str) -> Vec<String> {
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(_) => panic!("no such file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read!");
    let lines: Vec<String> = file_contents
        .split("\n")
        .map(|s: &str| s.to_string())
        .collect();
    lines
}

fn main() -> std::io::Result<()> {
    let should_check = std::env::args().nth(2);

    let mut should_check_str = "";
    if should_check != None {
        should_check_str = should_check.as_deref().unwrap();
    }

    let arg1 = std::env::args().nth(1);
    if arg1 == None {
        exit(1);
    }
    let arg1 = arg1.as_deref().unwrap();

    let mut check = 0;
    let mut check_file_w = 0;
    let mut check_file_r = 0;
    let mut filepath = "";
    if should_check_str == "checksum" {
        check = 1;
        filepath = arg1;
    } else if arg1 == "create-checksum-file" {
        check_file_w = 1;
    } else if arg1 == "checksum-file" {
        check_file_r = 1;
    } else {
        filepath = arg1;
    }

    let mut checksum = None; //either path or checksum
    if (check_file_w == 1 || check_file_r == 1) && checksum == None {
        checksum = std::env::args().nth(2);
    } else {
        checksum = std::env::args().nth(3);
    }
    let mut checksum_str = "";
    if checksum != None {
        checksum_str = checksum.as_deref().unwrap();
    }
    //unwrap and panic
    //println!("{} {} {} {:?}", check_file_r, check_file_w, check, checksum_str);

    let paths = std::fs::read_dir("./");

    // The available hash algorithms
    let hash_algorithms = vec!["SHA-256", "MD5", "SHA-1"];

    // Makes a selection menu in the terminal with the available algs
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&hash_algorithms)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let start = Instant::now();
    match selection {
        Some(index) => {
            if hash_algorithms[index] == "SHA-256" {
                // println!(
                //     "The SHA-256 checksum for the file is: {}",
                //     algorithms::calc_sha256(filepath)
                // );
                if check == 1 {
                    println!("{:?}", filepath);
                    if algorithms::calc_sha256(filepath) == checksum_str {
                        println!("It's a match");
                    } else {
                        println!("It doesn't match");
                    }
                }
                if check_file_w == 1 {
                    let fi = "sha256.checksum";
                    println!("Creating {}", fi);
                    let mut file = File::create(fi).expect("Creation Failed");
                    for p in paths? {
                        //println!("{}", path.unwrap().path().display());
                        let path = p?.path();
                        // Get path string.
                        let path_str = path.to_str().unwrap();
                        let path_str = &path_str[2..];
                        // if path_str.chars().nth(0).unwrap() == '.'{
                        //     continue;
                        // }
                        if std::fs::metadata(path_str).unwrap().is_dir() || path_str == fi {
                            //println!("Hello");
                            continue;
                        }
                        let code = algorithms::calc_sha256(path_str);
                        println!("{}", path_str);
                        //file.write_all(path_str.as_bytes())?;

                        file.write_all(code.as_bytes())?;
                        file.write_all(b"  ")?;
                        file.write_all(path_str.as_bytes())?;
                        file.write_all(b"\n")?;
                    }
                } else if check_file_r == 1 {
                    let fi = "sha256.checksum";
                    println!("Reading From {}", fi);
                    let lines = lines_from_file(fi);
                    let mut i = 0;
                    let mut j = 0;
                    for p in paths? {
                        let path = p?.path();
                        // Get path string.
                        let path_str = path.to_str().unwrap();
                        let path_str = &path_str[2..];

                        if std::fs::metadata(path_str).unwrap().is_dir() || path_str == fi {
                            i = i + 1;
                            continue;
                        }
                        let f_name = &lines[j][66..]; // 256 bits
                        //println!("{:?}", f_name);
                        if lines[j][..64] == algorithms::calc_sha256(path_str) && f_name == path_str
                        {
                            println!("{} matches", path_str);
                        } else {
                            println!("{} does not match", path_str);
                        }
                        j = j + 1;
                    }
                } else {
                    println!(
                        "The SHA-256 checksum for the file is: {}",
                        algorithms::calc_sha256(filepath)
                    );
                }
            } else if hash_algorithms[index] == "MD5" {
                if check == 1 {
                    //println!("{:?}", checksum_str);
                    if algorithms::calc_md5(filepath) == checksum_str {
                        println!("It's a match");
                    } else {
                        println!("It doesn't match");
                    }
                }
                if check_file_w == 1 {
                    let fi = "md5.checksum";
                    println!("Creating {}", fi);
                    let mut file = File::create(fi).expect("Creation Failed");
                    for p in paths? {
                        //println!("{}", path.unwrap().path().display());
                        let path = p?.path();
                        // Get path string.
                        let path_str = path.to_str().unwrap();
                        let path_str = &path_str[2..];
                        // if path_str.chars().nth(0).unwrap() == '.'{
                        //     continue;
                        // }
                        if std::fs::metadata(path_str).unwrap().is_dir() || path_str == fi {
                            continue;
                        }
                        println!("{}", path_str);
                        let code = algorithms::calc_md5(path_str);
                        file.write_all(code.as_bytes())?;
                        file.write_all(b"  ")?;
                        file.write_all(path_str.as_bytes())?;
                        file.write_all(b"\n")?;
                    }
                } else if check_file_r == 1 {
                    let fi = "md5.checksum";
                    println!("Reading From {}", fi);
                    let lines = lines_from_file(fi);
                    let mut i = 0;
                    let mut j = 0;
                    for p in paths? {
                        let path = p?.path();
                        // Get path string.
                        let path_str = path.to_str().unwrap();
                        let path_str = &path_str[2..];

                        if std::fs::metadata(path_str).unwrap().is_dir() || path_str == fi {
                            i = i + 1;
                            continue;
                        }
                        let f_name = &lines[j][34..]; // 128 bits
                        // println!("{:?}", algorithms::calc_md5(path_str));
                        // let ha = &lines[j][..32];
                        // println!("{:?}", ha);
                        if lines[j][..32] == algorithms::calc_md5(path_str) && f_name == path_str {

                            println!("{} matches", path_str);
                        } else {
                            println!("{} does not match", path_str);
                        }
                        i = i+1;
                        j = j+1;
                    }
                } else {
                    println!(
                        "The MD5 checksum for the file is: {}",
                        algorithms::calc_md5(filepath)
                    );
                }
            } else if hash_algorithms[index] == "SHA-1" {
                //println!("{}",check_file_w);
                if check == 1 {
                    //println!("{:?}", checksum_str);
                    if algorithms::calc_sha1(filepath) == checksum_str {
                        println!("It's a match");
                    } else {
                        println!("It doesn't match");
                    }
                }
                if check_file_w == 1 {
                    let fi = "sha1.checksum";
                    println!("Creating {}", fi);
                    let mut file = File::create(fi).expect("Creation Failed");
                    for p in paths? {
                        //println!("{}", path.unwrap().path().display());
                        let path = p?.path();
                        // Get path string.
                        let path_str = path.to_str().unwrap();
                        let path_str = &path_str[2..];
                        // if path_str.chars().nth(0).unwrap() == '.'{
                        //     continue;
                        // }
                        if std::fs::metadata(path_str).unwrap().is_dir() || path_str == fi {
                            continue;
                        }

                        println!("{}", path_str);
                        let code = algorithms::calc_sha1(path_str);
                        file.write_all(code.as_bytes())?;
                        file.write_all(b"  ")?;
                        file.write_all(path_str.as_bytes())?;
                        file.write_all(b"\n")?;
                    }
                } else if check_file_r == 1 {
                    let fi = "sha1.checksum";
                    println!("Reading From {}", fi);
                    let lines = lines_from_file(fi);
                    let mut i = 0;
                    let mut j = 0;
                    for p in paths? {
                        let path = p?.path();
                        // Get path string.
                        let path_str = path.to_str().unwrap();
                        let path_str = &path_str[2..];

                        if std::fs::metadata(path_str).unwrap().is_dir() || path_str == fi {
                            i = i + 1;
                            continue;
                        }

                        let f_name = &lines[j][42..]; // 160 bits
                        if lines[j][..40] == algorithms::calc_sha1(path_str) && f_name == path_str{

                            println!("{} matches", path_str);
                        } else {
                            println!("{} does not match", path_str);
                        }
                        j = j + 1;
                    }
                } else {
                    println!(
                        "The SHA1 checksum for the file is: {}",
                        algorithms::calc_sha1(filepath)
                    );
                }
            }
        }
        None => println!("Aborted: You did not select an algorithm"),
    }
    let duration = start.elapsed();
    println!("Elapsed Time: {:?}", duration);
    Ok(())
}
