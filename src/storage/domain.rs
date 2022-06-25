use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::NamedTempFile;

pub struct Storage {
    files: HashMap<String, PathBuf>,
}

impl Storage {
    // new storage with hash and file path
    fn new(files: HashMap<String, PathBuf>) -> Self {
        Storage { files: files }
    }
    fn get_file(&self, hash: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(file) = self.files.get(hash) {
            let data = fs::read(file)?;
            return Ok(data);
        }
        Err("No file found with hash xxx".into())
    }
    fn store(&self, data: Vec<u8>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_storage() {
        let storage = Storage::new(HashMap::from([(
            "abcd123".to_string(),
            PathBuf::from("/tmp/aux"),
        )]));
        assert_eq!(storage.files.len(), 1);
    }

    #[test]
    fn get_file() {
        let mut tmpfile = NamedTempFile::new().unwrap();
        write!(tmpfile, "Test data").unwrap();

        let storage = Storage::new(HashMap::from([(
            "abcd123".to_string(),
            PathBuf::from(tmpfile.path()),
        )]));

        let actual_data = storage.get_file("abcd123").unwrap();
        assert_eq!(String::from_utf8_lossy(&actual_data), "Test data");
    }
}
