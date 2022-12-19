pub use std::fs::File;
pub use std::io::{BufRead, BufReader, Lines, Write};
use std::process::Command;

pub fn command(name: &str, args: &[&str]) {
    Command::new(name)
        .args(args)
        .spawn()
        .expect("Expected Command To Run")
        .wait()
        .expect("Expected To Wait For Command");
}

pub fn path(year: u8, day: u8) -> String {
    "./input/".to_owned()
        + year.to_string().as_str()
        + "/"
        + if day < 10 { "0" } else { "" }
        + day.to_string().as_str()
        + ".txt"
}

pub fn lines(path: String) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Expected To Open File")).lines()
}

pub fn answer(answer: u32) -> usize {
    let answer = "\n".to_owned() + answer.to_string().as_str() + "\n\n";
    std::io::stdout()
        .write(answer.as_bytes())
        .expect("Expected To Write To Standard Out")
}
