use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

trait FileServer {
    fn new() -> Self;
    // returns file if available
    fn file(&self, hash: &'static str) -> Option<Vec<u8>>;
    // stores a fil
    fn store(&self, data: Vec<u8>) -> Result<(), Box<dyn Error>>;
}

struct SimpleServer {
    files: HashMap<String, File>,
}

impl FileServer for SimpleServer {
    fn new() -> Self {
        SimpleServer {
            files: HashMap::new(),
        }
    }
    fn file(&self, hash: &'static str) -> Option<Vec<u8>> {
        if let Some(file) = self.files.get(hash) {
            return None;
        }
        None
    }
    fn store(&self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
