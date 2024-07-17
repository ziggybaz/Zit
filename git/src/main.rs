use git;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1] == "init" {
        git::init();
        println!("Initialized git directory");
    } else if args[1] == "cat-file" {
        git::read_blob();
    } else if args[1] == "hash-object" && args[2] == "-w" {
        git::create_blob();
    } else {
        println!("unknown command: {}", args[1]);
    }
}
