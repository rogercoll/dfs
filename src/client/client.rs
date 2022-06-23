use crate::df::file::FileSegment;
use std::error::Error;
use std::fs::File;

struct Client {
    peer: String,
}

impl Client {
    fn get_file(hash: String) -> Result<File, Box<dyn Error>> {
        Err("f".into())
    }
    fn get_file_segment(hash: String) -> Result<FileSegment, Box<dyn Error>> {
        Err("f".into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_metadata_test() {}
}
