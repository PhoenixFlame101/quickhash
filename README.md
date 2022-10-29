# quickhash

---

`quickhash` is a cross-platform, open-source file hashing tool written in Rust. It is intended to calculate and verify checksums of files from the command line.

quickhash can calculate file hashes using the following cryptographic hash algorithms:

 - SHA-256
 - MD5
 - SHA-1

### Building

You can build and run the program using the command

    cargo run
    
To check the hash of a file, enter the file path of the file you want the checksum of:

    cargo run NOTICE
    The SHA-256 checksum for the file is: 675f53984423528d38403157e90b8b9df112f79a9249c22f49a889744eec4b23

Note: You need to have Rust installed to run this command.

##### This project is  maintained by [Abhinav Chennubhotla](https://github.com/PhoenixFlame101).
