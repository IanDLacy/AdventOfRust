mod basics;
mod commands;
mod y22;

use std::env;

fn main() {
    match env::args().nth(1).unwrap_or_default().as_str() {
        "b" => {
            commands::cargo_build();
        }
        "bc" => {
            commands::cargo_build();
            commands::git_commit();
        }
        "bcp" => {
            commands::cargo_build();
            commands::git_commit();
            commands::git_push();
        }
        "c" => {
            commands::git_commit();
        }
        "cp" => {
            commands::git_commit();
            commands::git_commit();
        }
        "p" => {
            commands::git_push();
        }
        "zsh" => {
            commands::zsh();
        }
        "1" => {
            commands::answer(y22::d01::p1());
            commands::answer(y22::d01::p2());
        }
        "2" => {
            commands::answer(y22::d02::p1());
            commands::answer(y22::d02::p2());
        }
        "3" => {
            commands::answer(y22::d03::p1());
            commands::answer(y22::d03::p2());
        }
        _ => {}
    }
}
