mod fs;
fn main(){
    let mut f = fs::MyFileSystem::new("disk0");
    // "cannot mutate immutable f" so need to make f mutable
    let mut buf = [0u8; 8];
    let test_string = "test".as_bytes();
    for i in 0..8 {
        buf[i] = test_string[i] as u8;
    }
    f.create_file(buf, 3);
}