use sha2::{Sha256, Digest};
use blake3;
use xxhash_rust::xxh3::Xxh3;
use std::path::Path;
use std::io::{BufReader, Read};
use std::fs::File;

#[allow(dead_code)]
pub enum HashAlgorithm {
    SHA256,
    Blake3,
    XXH3,
}

pub fn hash_calculate(path: &Path, hash_algo: &HashAlgorithm) -> Option<String>
{
    let file = File::open(path).ok()?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).ok()?;

    match hash_algo {
        HashAlgorithm::SHA256 => {
            let mut hasher = Sha256::new();
            hasher.update(&buffer);
            let hash_bytes = hasher.finalize();
            let mut hex_string =String::new();
            for byte in hash_bytes {
                hex_string.push_str(&format!("{:02x}", byte));
            }
            Some(hex_string)
        }
        
        HashAlgorithm::Blake3 => {
            let hash = blake3::hash(&buffer);
            let mut hex_string = String::new();
            for bytes in hash.as_bytes() {
                hex_string.push_str(&format!("{:02x}", bytes));
            }
            Some(hex_string)
        }

        HashAlgorithm::XXH3 => {
            let mut hasher = Xxh3::new();
            hasher.update(&buffer);
            let hash_value = hasher.digest();
            let hex_string = format!("{:016x}", hash_value);
            Some(hex_string)
        }
    }
}

//Unit testing
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_hash_calculate() {
        let path = Path::new("test_file.txt");
        let hash_algo = HashAlgorithm::SHA256;
        let hash = hash_calculate(&path, &hash_algo);
        assert!(hash.is_some())
    }
}



