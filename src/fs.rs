use std::{
    fs::File,
    io::{self, Seek, SeekFrom, Read, Write},
    str,
    mem,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::convert::TryFrom;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;


// pub const BLOCK_SIZE: i32 = 1024; // 1KB

#[derive(Serialize, Deserialize)]
pub struct IdxNode { // 'a is lifetime specifier
    // problem: length of array is always length of string? can't be fixed length?
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
            disk: OpenOptions::new().read(true).write(true).open(diskname).expect("There is no file to open. Did you run create_fs.rs first?")
        } 
        
    }

    pub fn create_file(&mut self, name: [u8; 8], size: i32) -> i32 {
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
        println!("Enough freeblocks, {freeblocks}, for the specified size");

        let mut nd =  IdxNode {
            name: [0u8; 8],
            size: -1, 
            block_pointers: [0; 8],
            used: -1
        };  
    
        let size_of_node = mem::size_of::<IdxNode>();
        // println!("SIZE OF INODE: {}", size_of_node);
        assert!(size_of_node == 48);
        let mut node_index = -1;
        // find unused inode, also check if inode exists with name already
        for i in (0..16).rev() { // iterator is u64 bruh
            self.disk.seek(SeekFrom::Start(u64::try_from(128 + (i * size_of_node)).expect("Conversion failed."))).expect("Failed to seek.");
            // CONSIDER: using bincode deserialize instead. But, this seems to work...
            nd = IdxNode::from_reader(&self.disk).expect("IdxNode read failed."); // lol just borrow?
            if nd.used == 0 {
                node_index = i as i32;
            } else if str::from_utf8(&nd.name).unwrap().eq(str::from_utf8(&name).unwrap()) { // question: from utf8 handles names < 8 size?
                println!("File found already in disk.");
                return -1
            }
        }
        if node_index == -1 { // if for loop never ran didn't read in an inode
            return -1;
        }
        nd.name = name; // pass by value? moves name, but name is array that implements Copy? 
        nd.size = size;
        nd.used = 1;

        let mut blocks_needed = size;
        for i in 0..128 {
            if blocks_needed == 0 {
                break;
            }
            if freelist[i] == 0 {
                freelist[i] = 1;
                // need to cast indexing to usize, since Index trait does not implement for i32
                nd.block_pointers[(size - blocks_needed) as usize] = i as i32; 
                blocks_needed -= 1;
            }
        }

        // seek to inode position
        self.disk.seek(SeekFrom::Start(u64::try_from(128 + ((node_index as usize) * size_of_node )).expect("Conversion failed."))).expect("Failed to seek.");
        
        // serialize inode as bytes in a buffer
        let inode_bytes = bincode::serialize(&nd).unwrap();
        
        // write buffer to disk
        self.disk.write(&inode_bytes).expect("Write failed.");

        // seek to beginning
        self.disk.seek(SeekFrom::Start(0)).expect("Failed to seek.");

        //write new freelist to disk
        self.disk.write(&freelist).expect("Write failed.");
        1
    }
    
    pub fn ls(&mut self) {
        let size_of_node = mem::size_of::<IdxNode>();
        assert!(size_of_node == 48);
        let mut nd =  IdxNode {
            name: [0u8; 8],
            size: -1, 
            block_pointers: [0; 8],
            used: -1
        };  
        
        for i in 0..16 {
            self.disk.seek(SeekFrom::Start(u64::try_from(128 + ((i as usize) * size_of_node)).expect("Conversion failed."))).expect("Failed to seek.");
            nd = IdxNode::from_reader(&self.disk).expect("IdxNode read failed."); 
            if nd.used == 1 {
                print!("{}, {} bytes. blocks: ", str::from_utf8(&nd.name).unwrap(), nd.size)
            }
            for j in 0..nd.size {
                print!("{} ", nd.block_pointers[j as usize]);
            }
            // WHY DOESN'T THIS WORK BUT ABOVE DOES??
            //print!("{}}}", nd.block_pointers[(nd.size - 1) as usize]); 
        }
    }

    pub fn delete_file(&mut self, name: [u8; 8]) -> i32 {
        println!("Deleting file {}", str::from_utf8(&name).unwrap());
        let size_of_node = mem::size_of::<IdxNode>();
        assert!(size_of_node == 48);

        self.disk.seek(SeekFrom::Start(0)).expect("Failed to seek.");
        let mut freelist = [0; 128]; // u8's size is 1 byte like a char
        self.disk.read_exact(&mut freelist).expect("Read failed.");

        let mut nd =  IdxNode {
            name: [0u8; 8],
            size: -1, 
            block_pointers: [0; 8],
            used: -1
        };  
        let mut node_index = -1;
        for i in 0..16 { 
            self.disk.seek(SeekFrom::Start(u64::try_from(128 + (i * size_of_node)).expect("Conversion failed."))).expect("Failed to seek.");
            nd = IdxNode::from_reader(&self.disk).expect("IdxNode read failed."); 
            if nd.used == 1 && str::from_utf8(&nd.name).unwrap().eq(str::from_utf8(&name).unwrap()) { // question: from utf8 handles names < 8 size?
                node_index = i as i32;
                for j in 0..nd.size {
                    // TODO: consider using usize explicitly instead of i32 when possible
                   
                    freelist[nd.block_pointers[j as usize] as usize] = 0;
                }
                nd.used = 0;
                break;
            }
        }
        if node_index == -1 {
            return -1;
        }

        self.disk.seek(SeekFrom::Start(u64::try_from(128 + ((node_index as usize) * size_of_node)).expect("Conversion failed."))).expect("Failed to seek.");
        let inode_bytes = bincode::serialize(&nd).unwrap();
        self.disk.write(&inode_bytes).expect("Write failed.");
        self.disk.seek(SeekFrom::Start(0)).expect("Failed to seek.");
        self.disk.write(&freelist).expect("Write failed.");
        1
    }
    
} 

    // delete_file
    // int delete_file(char name[8]);


    // read
    // int read(char name[8], int blockNum, char buf[1024]);

    // write
    // int write(char name[8], int blockNum, char buf[1024]);

    // close_disk
    // int close_disk();



