use std::env;
use std::process::Command;

pub fn command(name: &str, args: &[&str]) {
    Command::new(name)
        .args(args)
        .spawn()
        .expect("Expected Command To Run")
        .wait()
        .expect("Expected To Wait For Command");
}

pub fn zsh() {
    let dir = env::current_dir().expect("Expected Current Directory");
    let dir = dir
        .to_str()
        .expect("Expected Successful Cast To String Slice");

    let path =
        env::var("PATH").expect("Expected Environment Variable") + ":" + dir + "/target/debug";

    env::set_var("PATH", path);

    command("zsh", &["-l"]);
}

pub fn cargo_build() {
    command("cargo", &["build"]);
}

pub fn git_commit() {
    command("git", &["add", "."]);
    command("git", &["commit"]);
}

pub fn git_push() {
    command("git", &["push"]);
}
