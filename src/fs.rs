use std::io::SeekFrom;
use std::{
    fs::File,
    io::{self, Seek, Read},
};
use byteorder::{LittleEndian, ReadBytesExt};



pub const BLOCK_SIZE: i32 = 1024; // 1KB


pub struct IdxNode { // 'a is lifetime specifier
    name: [u8; 8], // array of bytes. Alternatively: [&'a str; 8]?
    size: i32,
    block_pointers: [i32; 8], // design change: initialize all to -1...
    used: i32,
}

impl IdxNode {
    // reads from file field by field to avoid using unsafe transmute. 
    fn from_reader(mut rdr: impl Read) -> io::Result<Self> {
        let mut name = [0u8; 8]; // initialize as 0s...how does this work?
        rdr.read_exact(&mut name)?;
        let size = rdr.read_i32::<LittleEndian>()?;
        let mut block_pointers: [i32;8] = [-1;8]; // b/c we have size, theoretically shouldn't ever run into -1
        for i in 0..8 {
            block_pointers[i] = rdr.read_i32::<LittleEndian>()?;
        }
        let used = rdr.read_i32::<LittleEndian>()?;
        Ok(IdxNode {
            name,
            size,
            block_pointers,
            used,
        })
    }
}


// -------------------------NOTES----------------------------
// https://github.com/umass-cs-377/377-project-filesystem/blob/master/docs/index.md
// https://blog.carlosgaldino.com/writing-a-file-system-from-scratch-in-rust.html
// https://www.reddit.com/r/rust/comments/9gc75o/how_to_get_versatile_file_handling_like_c/
// -----------------------------------------------------------

// class myFileSystem
pub struct MyFileSystem {
    disk: File,
}

impl MyFileSystem { // impl is kinda like a class, implements functions for struct
    pub fn new(diskname: &str) -> Self { // diskname: size 16
        Self { 
            disk: File::open(diskname).expect("There is no file to open.")
        } 
        
    }
    // create_file, name: size 8
    // Step 1: Check to see if we have sufficient free space on disk by reading in the free block list. To do this:
    // Move the file pointer to the start of the disk file.
    // Read the first 128 bytes (the free/in-use block information)
    // Scan the list to make sure you have sufficient free blocks to allocate a new file of this size

    // Step 2: we look for a free inode on disk
    // Read in an inode
    // Check the "used" field to see if it is free
    // If not, repeat the above two steps until you find a free inode
    // Set the "used" field to 1
    // Copy the filename to the "name" field
    // Copy the file size (in units of blocks) to the "size" field

    // Step 3: Allocate data blocks to the file
    // for(i=0;i<size;i++)
    // Scan the block list that you read in Step 1 for a free block
    // Once you find a free block, mark it as in-use (Set it to 1)
    // Set the blockPointer[i] field in the inode to this block number.
    // end for

    // Step 4: Write out the inode and free block list to disk
    // Move the file pointer to the start of the disk file
    // Write out the 128 byte free block list to the disk file
    // Move the file pointer to the position on disk where this inode was stored
    // Write out the inode to the disk file
    
    pub fn create_file(&mut self, name: &str, size: i32) -> i32 {
        if size < 1 || size > 8 {
            println!("Size specified is nonpositive or greater than max");
            return -1;  // needs return to end early
        } 
        println!("Creating file!");

        // Move to beginning of disk
        self.disk.seek(SeekFrom::Start(0)).expect("Failed to seek.");
       
        // Read first 128 bytes to freelist array to initialize it
        // buffer (freelist) MUST be initialized before a file reads to it. 
        let mut freelist = [0; 128]; // u8's size is 1 byte like a char
        // read_exact requires a mutable buf
        self.disk.read_exact(&mut freelist).expect("Read failed.");
        

        // count number if freeblocks to ensure there's enough for the size
        let mut freeblocks = 0;
        for i in 0..128 {
            freeblocks += if freelist[i] == 1 {0} else {1};
        }
        if freeblocks < size {
            return -1;
        }
        println!("Enough freeblocks: {freeblocks}, for the specified size");

        // find unused inode, also check if inode exists with name already
        
        // how to read inode from disk? 
        // 1. unsafe + transmute
        // 2. bincode, serde
        // 3. byteorder
        let mut nd: IdxNode;
        // assert size of IdxNode is 48...
        // then use sizeof var for later
        let Size_of_IdxNode = 48;
        let mut ndidx:i32 = -1;
        for i in (0i32..16).rev() { // iterator is u64 bruh
            self.disk.seek(SeekFrom::Start(128 + (i * Size_of_IdxNode))).expect("Failed to seek.");
            nd = IdxNode::from_reader(&self.disk).expect("IdxNode read failed."); // lol just borrow?
            if nd.used == 0 {
                ndidx = i;
            }
        }
        0

    }
        
} 

    // delete_file
    // int delete_file(char name[8]);


    // read
    // int read(char name[8], int blockNum, char buf[1024]);

    // write
    // int write(char name[8], int blockNum, char buf[1024]);


    // ls
    // int ls();

    // close_disk
    // int close_disk();



