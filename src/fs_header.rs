pub const BLOCK_SIZE: i32 = 1024; // 1KB


pub struct IdxNode {
    name: &str, // str or string?
    size: int32,
    blockPointers: [i32; 8],
    used: int32,
}