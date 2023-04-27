// This file is responsible for making and initializing the disk!

// step 1: check usage of command call. Ensure name of new file is present

// step 2: create 128 KB file in directory 
// In C++, this is done by using fstream Open(), with specified flags: write read, creation, truncate, user has read and write permission
// In Rust, we can create file some other way, but need to ensure read/write permission and create 
// there's the OpenOptions method

// step 3: Big picture: initialize first KB as superblock. 
// superblock:
// first 128 bytes is free block list. First block is 1 because of superblock, rest is 0.
// after is the 16 inodes, one for each file
// Implementation: 
// use buffer of 1024 size with 1 at first index, then write to disk

// step 4: zero out all other data blocks
// Implementation:
// using same buffer, with 0 at first index, write to rest of disk
