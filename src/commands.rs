use super::basics;

pub fn zsh() {
    basics::path();
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
