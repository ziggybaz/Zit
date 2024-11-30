extern crate flate2;
extern crate hex;

use std::{
    io::{ Result, Write }, 
    fs,
    ffi,
    env,
    path::Path
};
use sha1::{Sha1, Digest };
use flate2::Compression;
use flate2::write::ZlibEncoder;


pub fn write_tree() {
    let cwd = env::current_dir().unwrap();

    match write_tree_recursive(&cwd) {
        Ok(tree_sha) => {
            println!("{}", tree_sha);
        }
        Err(err) => {
            eprintln!("Error writing tree: {}", err)
        }
    }
}

fn write_tree_recursive(directory: &Path) -> Result<String> {
    let mut entries = Vec::new();

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();

        if name == ".git" {
            continue;
        }

        if path.is_file() {
            let blob_sha = create_blob_from_path(&path)?;
            let mode = b"100644";
            entries.push(encode_tree_entry(mode, &name, &hex_to_bytes(&blob_sha)));
        } else if path.is_dir() {
            let tree_sha = write_tree_recursive(&path)?;
            let mode = b"40000";
            entries.push(encode_tree_entry(mode, &name, &hex_to_bytes(&tree_sha)));
        }
    }

    let content: Vec<u8> = entries.into_iter().flatten().collect();
    let header = format!("tree {}\0", content.len());
    let mut object = Vec::new();
    object.extend(header.as_bytes());
    object.extend(content);

    let sha = calculate_sha1(&object);

    write_object(&sha, &object)?;

    Ok(sha)
}

fn encode_tree_entry(mode: &[u8], name: &ffi::OsStr, sha: &[u8]) -> Vec<u8> {
    let mut entry = Vec::new();

    entry.extend_from_slice(mode);
    entry.push(b' ');
    entry.extend_from_slice(name.to_string_lossy().as_bytes());
    entry.push(0);
    entry.extend_from_slice(sha);

    entry
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex::decode(hex).expect("Lol, Invalid hex string bruv...")
}

fn calculate_sha1(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);

    format!("{:x}", hasher.finalize())
}

fn write_object(sha: &str, content: &[u8]) -> Result<()> {
    let object_dir = format!(".git/objects/{}", &sha[..2]);
    let object_file = format!("{}/{}", object_dir, &sha[2..]);

    if fs::metadata(&object_file).is_ok() {
        return Ok(());
    }

    fs::create_dir_all(&object_dir)?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;
    let compressed_data = encoder.finish()?;

    fs::write(object_file, compressed_data)?;

    Ok(())
}

fn create_blob_from_path(file_path: &Path) -> Result<String> {
    let data = fs::read(file_path)?;
    let header = format!("blob {}\0", data.len());
    let mut object = Vec::new();
    object.extend(header.as_bytes());
    object.extend(&data);

    let sha = calculate_sha1(&object);

    write_object(&sha, &object)?;

    Ok(sha)
}
