use git::{initialization, cat_file, hash_object, ls_tree};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        initialization::init();
        println!("Initialized git directory");
    } else if args[1] == "cat-file" {
        cat_file::read_blob();
    } else if args[1] == "hash-object" && args[2] == "-w" {
        hash_object::create_blob();
    } else {
        println!("unknown command: {}", args[1]);
    }
}
