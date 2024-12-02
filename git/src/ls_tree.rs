use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::Cursor;
use flate2::read::ZlibDecoder;
use std::{fs, env, process};

pub fn read_tree(){
		let args: Vec<String> = env::args().collect();
		let tree_hash = &args[3];
		let object_dir = format!(".git/objects");//where I was initially messing up, was reading from the wrong path '.git/'
		let tree_object_path = format!("{}/{}/{}", object_dir, &tree_hash[..2], &tree_hash[2..]);

		//this line saved me a week of frustration.
		eprintln!("File path problem: reading tree object from `{}`", tree_object_path);

		let tree_object = fs::read(&tree_object_path).unwrap_or_else(|error| {
			if error.kind() == ErrorKind::NotFound {
				panic!("File not present mate!!!")
			} else { panic!("UNable to read data from the file...") }
		});

		let decompressed_tree = match decode(&tree_object[..]) {
			Ok(data) => data,
			Err(_) => {
				eprintln!("Failed to decompress");
				process::exit(1);
			}
		};

		let mut index = decompressed_tree.iter().position(|&x| x == 0).expect("unable to iterate over the decompressed tree binary data") + 1;
		while index < decompressed_tree.len() {
			let space_character_index = decompressed_tree[index..].iter().position(|&x| x == b' ').expect("unable to iterate over the space character");
			let null_byte_index = decompressed_tree[index + space_character_index..]
			.iter()
			.position(|&x| x == 0)
			.unwrap();

			let name = &decompressed_tree[index + space_character_index + 1..index + space_character_index + null_byte_index];

			println!("{}", String::from_utf8_lossy(name));

			index += space_character_index + null_byte_index + 23;
		} 
}

fn decode(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
		let mut decoder = ZlibDecoder::new(Cursor::new(data));
		let mut decompressed_data = Vec::new();
		decoder.read_to_end(&mut decompressed_data)?;
		Ok(decompressed_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flate2::write::ZlibEncoder;
    use tempfile::NamedTempFile;
    use flate2::Compression;
    use std::io::Write;
    use std::fs::write;

    #[test]
    fn test_decode() {
        let test_data = b"Gachagua impeached";

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(test_data).expect("shiezer, failed to encode");
        let compressed_data = encoder.finish().expect("shiezer, failed to encode");

        //so this is where we test if the dang decoder works as we've just encoded above.
        let decoded_data = decode(&compressed_data).expect("tired of this expects");

        // moment of trurh
        assert_eq!(decoded_data, test_data);
    }

    #[test]
    fn test_read_tree() { // a real pain to try and test using unit
        unimplemented!()
    }
}
