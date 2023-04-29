mod fs;
fn main(){
    let mut f = fs::MyFileSystem::new("disk0");
    // cannot mutate immutable f so need to make f mutable
    f.create_file("test", 3);
}