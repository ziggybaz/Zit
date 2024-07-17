extern crate flate2;

use std::{env, fs};
use std::io::prelude::*;
use std::io::ErrorKind;
use flate2::read::ZlibDecoder;

pub fn read_blob() {
    let args: Vec<String> = env::args().collect();
    let data: Vec<u8> = fs::read(format!(".git/objects/{}/{}", &args[3][..2], &args[3][2..])).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            panic!("Unable to read from file...");
        } else { Vec::new() }
    });
    
    decompress_data(&data);
}

fn decompress_data(obj: &Vec<u8>) {
    let mut decompress_data = ZlibDecoder::new(&obj[..]);
    let mut s = String::new();

    decompress_data.read_to_string(&mut s).expect("Unable to read to string.");
    print!("{}", &s[8..]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_blob() {
        unimplemented!();
    }

    #[test]
    fn test_decopressed_data() {
        unimplemented!();
    }

}
