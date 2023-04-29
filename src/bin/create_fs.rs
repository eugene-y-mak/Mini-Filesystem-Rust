use std::env;
use std::io::Write;
use std::process;
use std::fs::OpenOptions;

// This file is responsible for making and initializing the disk!


fn main() { // takes in name of file and diskname 
    // step 1: check usage of command call. Ensure name of new file is present

    // MUST annotate this, or else collect() won't know what to return
    let args: Vec<String> = env::args().collect(); // env::args() returns an iterator
    let cmd = &args[0];
    if args.len() <= 1 {
        println!("usage: {cmd} <diskFileName>");
        process::exit(0);
    }
    
    let disk_name = &args[1];
    println!("Creating a 128KB file");
    println!("This file will act as a dummy disk and will hold your filesystem");
    // step 2: create 128 KB file in directory 
    // In C++, this is done by using fstream Open(), with specified flags: write read, creation, truncate, user has read and write permission
    // In Rust, we can create file some other way, but need to ensure read/write permission and create 
    // "Sets the option to create a new file, or open it if it already exists."
    // File should automatically close, probably
    // unpack Result using .expect()
    let mut disk = OpenOptions::new().write(true).create(true).open(disk_name).expect("Disk open AND creation failed.");
    
    println!("Formatting your filesystem...");
    // step 3: Big picture: initialize first KB as superblock. 
    // superblock:
    // first 128 bytes is free block list. First block is 1 because of superblock, rest is 0.
    // after is the 16 inodes, one for each file
    // Implementation: 
    // use buffer of 1024 size with 1 at first index, then write to disk
    let mut buf = [0; 1024];
    buf[0] = 1;
    disk.write(&buf).expect("Write failed.");

    
    // step 4: zero out all other data blocks
    // Implementation:
    // using same buffer, with 0 at first index, write to rest of disk, the remaining 127 data blocks
    buf[0] = 0;
    for _n in 0..127 {
        disk.write(&buf).expect("Write failed.");
    }

}