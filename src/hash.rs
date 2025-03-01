use sha2::{Sha256, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::error::Error;

pub fn hash_file_sha256<P: AsRef<Path>>(path: P) -> Result<String, Box<dyn Error>> {
    // Open the file in read-only mode.
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    
    // Create a Sha256 object
    let mut hasher = Sha256::new();

    // Read the file in chunks so we don't need to load the entire file into memory.
    let mut buffer = [0; 1024 * 8]; // 8 KB buffer
    loop {
        let bytes_read = reader.read(&mut buffer)?;
        if bytes_read == 0 {
            break; // end of file
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Retrieve hash digest as array of bytes
    let result = hasher.finalize();

    // Convert to hexadecimal string representation
    Ok(hex::encode(result))
}