pub use std::fs::File;
pub use std::io::{BufRead, BufReader, Lines};

pub fn get_lines(path: &str) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Expected To Open File")).lines()
}
