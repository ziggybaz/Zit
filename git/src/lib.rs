extern crate flate2;

use std::io::prelude::*;
use std::{env, fs};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use sha1::{Sha1, Digest};

pub fn init() {
    fs::create_dir(".git").unwrap();
    fs::create_dir(".git/objects").unwrap();
    fs::create_dir(".git/refs").unwrap();
    fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
}

pub fn read_blob() {
    let args: Vec<String> = env::args().collect();
    let data: Vec<u8> = fs::read(format!(".git/objects/{}/{}", &args[3][..2], &args[3][2..])).unwrap();
    let mut decompress_data = ZlibDecoder::new(&data[..]);
    let mut s = String::new();

    decompress_data.read_to_string(&mut s).unwrap();
    print!("{}", &s[8..]);
}

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































