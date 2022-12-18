use std::io::Write;
use std::process::Command;

pub fn answer(answer: u32) -> usize {
    let answer = "\n".to_owned() + answer.to_string().as_str() + "\n\n";
    std::io::stdout()
        .write(answer.as_bytes())
        .expect("Expected To Write To Standard Out")
}

pub fn cargo_build() {
    Command::new("cargo")
        .arg("build")
        .spawn()
        .expect("Expected Command To Run")
        .wait()
        .expect("Expected To Wait For Command");
}

pub fn git_commit() {
    Command::new("git")
        .arg("add")
        .arg(".")
        .spawn()
        .expect("Expected Command To Run")
        .wait()
        .expect("Expected To Wait For Command");
    Command::new("git")
        .arg("commit")
        .spawn()
        .expect("Expected Command To Run")
        .wait()
        .expect("Expected To Wait For Command");
}

pub fn git_push() {
    Command::new("git")
        .arg("push")
        .spawn()
        .expect("Expected Command To Run")
        .wait()
        .expect("Expected To Wait For Command");
}
