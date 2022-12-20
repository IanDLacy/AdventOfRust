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
            commands::git_push();
        }
        "p" => {
            commands::git_push();
        }
        "r" => {
            commands::cargo_run();
        }
        "zsh" => {
            commands::zsh();
        }
        "2201" => {
            basics::answer(y22::d01::p1());
            basics::answer(y22::d01::p2());
        }
        "2202" => {
            basics::answer(y22::d02::p1());
            basics::answer(y22::d02::p2());
        }
        "2203" => {
            basics::answer(y22::d03::p1());
            basics::answer(y22::d03::p2());
        }
        "2204" => {
            basics::answer(y22::d04::p1());
            basics::answer(y22::d04::p2());
        }
        "2205" => {
            basics::answer(y22::d05::p1());
            basics::answer(y22::d05::p2());
        }
        _ => {}
    }
}
