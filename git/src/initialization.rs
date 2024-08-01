use std::fs;

pub fn init() {
    fs::create_dir(".git").unwrap_or_else(|error| { panic!("Unable to create .git directory!!!\nError: {error:?}") });
    fs::create_dir(".git/objects").unwrap_or_else(|error| { panic!("Unable to create objects directory!!!\nError: {error:?}") });
    fs::create_dir(".git/refs").unwrap_or_else(|error| { panic!("Unable to greate refs directory!!!\nError: {error:?}")} );
    fs::write(".git/HEAD", "ref: refs/heads/main\n").expect("Unable to write to the file, tray again");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    
    #[test]
    fn initialization() {
        let git_directory = ".git";
        let head_content = fs::read_to_string(".git/HEAD").unwrap();
        init();
    
        //should have done set-up by removing existing '.git' dir but didn't as I don't want it to delete the native one otherwise for the test case add a set-up to check for and remove existing '.git' dir if not test itafail na
        //hutajua mbona, do as I say.
        assert!(Path::new(git_directory).exists());
        assert!(Path::new(".git/objects").exists());
        assert!(Path::new(".git/refs").exists());
        assert_eq!(head_content, "ref: refs/heads/main\n");

        fs::remove_dir_all(git_directory).expect("Failed to perform test tear down...");
    }
}
