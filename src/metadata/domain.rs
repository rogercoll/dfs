use std::convert::From;
use std::fs::File;

struct FileSegment {
    index: u64,
    hash: String,
}

struct FileMetadata {
    merkle_root: String,
    segments: Vec<FileSegment>,
}

impl From<File> for FileMetadata {
    fn from(file: File) -> Self {
        FileMetadata {
            merkle_root: "abc".into(),
            segments: vec![],
        }
    }
}
