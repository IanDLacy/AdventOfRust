mod modules;

use modules::*;
use std::env;

fn main() {
    match env::args().nth(1).unwrap_or_default().as_str() {
        "b" => {
            commands::cargo_build();
        }
        "c" => {
            commands::git_commit();
        }
        "p" => {
            commands::git_push();
        }
        "bcp" => {
            commands::cargo_build();
            commands::git_commit();
            commands::git_push();
        }
        "1" => {
            commands::answer(d01::p1());
            commands::answer(d01::p2());
        }
        "2" => {
            commands::answer(d02::p1());
            commands::answer(d02::p2());
        }
        "3" => {
            commands::answer(d03::p1());
            commands::answer(d03::p2());
        }
        _ => {}
    }
}
