use std::io::SeekFrom;
use std::fs::File;
use std::io::Seek;


pub const BLOCK_SIZE: i32 = 1024; // 1KB
pub struct IdxNode {
    name: String, // str or string?
    size: i32,
    block_pointers: [i32; 8],
    used: i32,
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
    
    pub fn create_file(&mut self, name: &str, size: i32) {
        println!("creating file!");
        self.disk.seek(SeekFrom::Start(0)).expect("Failed to seek.");
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
}


