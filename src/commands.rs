use std::env;

use super::basics;

pub fn zsh() {
    let dir = env::current_dir()
        .expect("Expected Current Directory")
        .join("target/debug");
    let path = env::var_os("PATH").expect("Expected Environment Variable");
    let mut paths = env::split_paths(&path).collect::<Vec<_>>();
    paths.push(dir);
    env::set_var(
        "PATH",
        env::join_paths(paths).expect("Expected To Join Paths"),
    );
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
