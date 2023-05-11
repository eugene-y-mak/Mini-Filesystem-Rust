mod fs;
use std::str;
fn main(){
    let mut f = fs::MyFileSystem::new("disk0");
    // "cannot mutate immutable f" so need to make f mutable
    let mut test_buf = [0u8; 8];
    let test_string = "test".as_bytes();
    for i in 0..4 {
        test_buf[i] = test_string[i] as u8;
    }
    f.create_file(test_buf, 4);
    f.ls();
    let mut buf = [3u8; 1024];
    f.write(test_buf, 0, buf);
    let mut read_buf = [0u8; 1024];
    f.read(test_buf, 0, &mut read_buf); // THE REFERENCE NEEDS TO BE MUTABLE
    println!("BUFFER READ CHECK:");
    for i in 0..1024 {
        print!("{}", read_buf[i]);
    }
    println!("\n");
    f.delete_file(test_buf);
    f.ls();
    let return_code = f.close_disk();
    println!("Close disk returned: {}", return_code);

}