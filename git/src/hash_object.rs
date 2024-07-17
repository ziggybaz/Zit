extern crate flate2;
extern crate hex;

use std::io::prelude::*;
use std::{env, fs};
use std::io::ErrorKind;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Sha1, Digest};

pub fn create_blob() {
    let args: Vec<String> = env::args().collect();
    let data = read_data_from_file(&args[3]).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("File does not exist in the file system!!!😪")
        } else { panic!("Sorry mate. Unable to read data from file...😫") }
    });
    let hash_hex = compute_hash(&data);
    let compressed_data = compress_data(&data);
    store_data_in_object_store(&hash_hex, &compressed_data);

    print!("{}", hash_hex);
}

fn read_data_from_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    Ok(fs::read(file_path)?)
}

fn compute_hash(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    let final_data_string = format!("blob {}\0{}", data.len(), String::from_utf8_lossy(data));
    hasher.update(final_data_string.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn compress_data(data: &[u8]) -> Vec<u8> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().expect("Encoder failed to finish data compression")
}

fn store_data_in_object_store(hash_hex: &str, compressed_data: &[u8]) {
    let hash_bytes = hex::decode(&hash_hex[0..2]).unwrap();
    let data_directory = format!(".git/objects/{:02x}", hash_bytes[0]);
    let data_file = format!("{}/{}", data_directory, &hash_hex[2..]);

    fs::create_dir_all(&data_directory).unwrap();
    fs::write(&data_file, compressed_data).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::AlreadyExists {
            panic!("Such a file already exists in the file system...😏");
        } else { panic!("Unable to write to file...😪") }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_blob() {
        unimplemented!();
    }

    #[test]
    fn test_read_data_from_file() {
        unimplemented!();
    }
    #[test]
    fn test_compute_hash() {
        unimplemented!();
    }

    #[test]
    fn test_store_data_in_object_store() {
        unimplemented!();
    }
}













