use std::collections::HashMap;

// smallest unit that represents a part of a file
pub struct FileSegment {
    // bytes of the file segment
    data: Vec<u8>,
}

struct FileMetadata {
    merkle_root: String,
    // hash and sequence number of the segment
    segments_hash: HashMap<String, u32>,
}

struct File {
    segments: Vec<FileSegment>,
}
