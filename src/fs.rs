mod fs_header;

// -------------------------NOTES----------------------------
// https://github.com/umass-cs-377/377-project-filesystem/blob/master/docs/index.md
// https://blog.carlosgaldino.com/writing-a-file-system-from-scratch-in-rust.html
// https://www.reddit.com/r/rust/comments/9gc75o/how_to_get_versatile_file_handling_like_c/
// -----------------------------------------------------------

// class myFileSystem
// Can use traits or impl Struct
struct myFileSystem {
    disk: File,
}

impl myFileSystem { // impl is kinda like a class
    fn new(diskname: &str) -> Self { // diskname: size 16
        // Self { diskname } Probably don't need this because not setting diskname, but using it
        disk = File::open(diskname)?;
    }
    // create_file, name: size 8
    //int create_file(char name[8], int size);
    fn create_file(name: &str, size: int) {
        
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


