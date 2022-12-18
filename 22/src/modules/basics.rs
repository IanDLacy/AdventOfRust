pub use std::fs::File;
pub use std::io::{BufRead, BufReader, Lines};

pub fn path(day: u8) -> String {
    "./input/".to_owned() + if day < 10 { "0" } else { "" } + day.to_string().as_str() + ".txt"
}

pub fn lines(path: String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Expected To Open File")).lines()
}
