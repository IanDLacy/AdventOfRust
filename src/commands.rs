use std::env;

use super::basics;

pub fn zsh() {
    let dir = env::current_dir().expect("Expected Current Directory");
    let dir = dir
        .to_str()
        .expect("Expected Successful Cast To String Slice");

    let path =
        env::var("PATH").expect("Expected Environment Variable") + ":" + dir + "/target/debug";

    env::set_var("PATH", path);

    basics::command("zsh", &["-l"]);
}

pub fn cargo_build() {
    basics::command("cargo", &["build"]);
}

pub fn git_commit() {
    basics::command("git", &["add", "."]);
    basics::command("git", &["commit"]);
}

pub fn git_push() {
    basics::command("git", &["push"]);
}
