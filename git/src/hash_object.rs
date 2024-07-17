extern crate flate2;

use std::io::prelude::*;
use std::{env, fs};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Sha1, Digest};

pub fn create_blob() {
    let args: Vec<String> = env::args().collect();
    let mut hasher = Sha1::new();
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    let data = fs::read(&args[3]).unwrap();
    let final_data_string = format!("blob {}\0{}", data.len(), String::from_utf8_lossy(&data));

    hasher.update(final_data_string.as_bytes());
    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);
    print!("{}", hash_hex);

    encoder.write_all(final_data_string.as_bytes()).unwrap();
    let compressed_data = encoder.finish().unwrap();

    let data_directory = format!(".git/objects/{:02x}", &result[0]);
    let data_file = format!("{}/{}", data_directory, &hash_hex[2..]);
    fs::create_dir_all(&data_directory).unwrap();
    fs::write(&data_file, &compressed_data).unwrap();
}
