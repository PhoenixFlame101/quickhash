use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::process::exit;

mod algorithms;

fn main() -> std::io::Result<()> {
    let should_check = std::env::args().nth(2);
    let mut should_check_str = "";
    if should_check != None {
        should_check_str = should_check.as_deref().unwrap();
    }
    let mut check = 0;
    if should_check_str == "checksum"{
       check = 1; 
    }
    let checksum = std::env::args().nth(3);
    let mut checksum_str = "";
    if checksum != None {
        checksum_str = checksum.as_deref().unwrap();
    }
    
    //println!("{:?}",should_check_str);
    let filepath = std::env::args().nth(1);
    if filepath == None {
        exit(1);
    }
    let filepath = filepath.as_deref().unwrap();
    

    // The available hash algorithms
    let hash_algorithms = vec!["SHA-256", "MD5", "SHA1"];

    // Makes a selection menu in the terminal with the available algs
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&hash_algorithms)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => {
            if hash_algorithms[index] == "SHA-256" {
                println!(
                    "The SHA-256 checksum for the file is: {}",
                    algorithms::calc_sha256(filepath)
                );
                if check == 1 {
                    //println!("{:?}", checksum_str);
                    if algorithms::calc_sha256(filepath) == checksum_str {
                        println!("It's a match");
                    }
                    else{
                        println!("It doesn't match");
                    }
                }
            } else if hash_algorithms[index] == "MD5" {
                println!(
                    "The MD5 checksum for the file is: {}",
                    algorithms::calc_md5(filepath)
                );
                if check == 1 {
                    //println!("{:?}", checksum_str);
                    if algorithms::calc_md5(filepath) == checksum_str {
                        println!("It's a match");
                    }
                    else{
                        println!("It doesn't match");
                    }
                }
            } else if hash_algorithms[index] == "SHA1" {
                println!(
                    "The SHA1 checksum for the file is: {}",
                    algorithms::calc_sha1(filepath)
                );
                if check == 1 {
                    //println!("{:?}", checksum_str);
                    if algorithms::calc_sha1(filepath) == checksum_str {
                        println!("It's a match");
                    }
                    else{
                        println!("It doesn't match");
                    }
                }
            }
        }
        None => println!("Aborted: You did not select an algorithm"),
    }

    Ok(())
}
