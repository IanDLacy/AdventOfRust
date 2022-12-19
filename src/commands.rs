use std::env;
use std::process::Command;

pub fn zsh() {
    let dir = env::current_dir().expect("Expected Current Directory");
    let dir = dir
        .to_str()
        .expect("Expected Successful Cast To String Slice");

    let path =
        env::var("PATH").expect("Expected Environment Variable") + ":" + dir + "/target/debug";

    env::set_var("PATH", path);

    Command::new("zsh")
        .spawn()
        .expect("Expected Command To Run")
        .wait()
        .expect("Expected To Wait For Command");
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
