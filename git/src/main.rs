use git::{initialization, cat_file, hash_object, ls_tree};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = Command::from_args(&args);

    match command {
        Command::Init => {
            initialization::init();
            println!("Initialied git directory");
        },
        Command::CatFile => cat_file::read_blob(),
        Command::HashObject => hash_object::create_blob(),
        Command::Invalid(cmd) => println!("Argument NOT supported: {cmd}"),
    }
}

enum Command {
    Init,
    CatFile,
    HashObject,
    Invalid(String),
}

impl Command {
    fn from_args(args: &[String]) -> Self {
        match args {
            [_, cmd] if cmd == "init" => Command::Init,
            [_, cmd, ..] if cmd == "cat-file" => Command::CatFile,
            [_, cmd, option, ..] if cmd == "hash-object" && option == "-w" => Command::HashObject,
            _ => Command::Invalid("".to_string()),
        }
    }
}
