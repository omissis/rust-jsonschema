use crate::cli::Command as CliCommand;
use crate::loader::{FileLoader, Loader};
use std::path::PathBuf;

pub struct Command {
    pub file: PathBuf,
}

impl CliCommand for Command {
    fn run(&self) {
        if *self.file != PathBuf::default() {
            let filepath = self.file.to_str().unwrap();

            let loadr = FileLoader::new();

            let res = loadr.load(filepath);

            println!("Generating from {filepath}");
            println!("{:?}", res);
        } else {
            println!("Generating from empty file")
        }
    }
}
