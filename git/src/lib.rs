extern crate flate2;

use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use std::{env, fs};

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
