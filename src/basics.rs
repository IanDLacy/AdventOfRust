pub use std::fs::File;
pub use std::io::{BufRead, BufReader, Lines, Write};

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn path() {
    let dir = env::current_dir()
        .expect("Expected Current Directory")
        .join("target/debug");
    let path = env::var_os("PATH").expect("Expected Environment Variable");
    let mut paths = env::split_paths(&path).collect::<Vec<PathBuf>>();
    paths.push(dir);
    env::set_var(
        "PATH",
        env::join_paths(paths).expect("Expected To Join Paths"),
    );
}

pub fn command(name: &str, args: &[&str]) {
    Command::new(name)
        .args(args)
        .spawn()
        .expect("Expected Command To Run")
        .wait()
        .expect("Expected To Wait For Command");
}

pub fn input(year: u8, day: u8) -> PathBuf {
    let mut path = PathBuf::from("./input/")
        .join(year.to_string())
        .join(Path::new(
            &(String::from(if day < 10 { "0" } else { "" }) + &day.to_string()),
        ));
    path.set_extension("txt");
    path
}

pub fn lines(path: PathBuf) -> Lines<BufReader<File>> {
    BufReader::new(File::open(path).expect("Expected To Open File")).lines()
}

pub fn answer(answer: String) -> usize {
    let answer = "\n".to_owned() + answer.as_str() + "\n\n";
    std::io::stdout()
        .write(answer.as_bytes())
        .expect("Expected To Write To Standard Out")
}
