mod modules;

use std::env;

fn main() {
    match env::args().nth(1).unwrap_or_default().as_str() {
        "go" => {
            modules::commands::cargo_build();
            modules::commands::git_add();
            modules::commands::git_commit();
            modules::commands::git_push();
        }
        "1" => {
            modules::commands::answer(modules::d01::p1());
            modules::commands::answer(modules::d01::p2());
        }
        "2" => {
            modules::commands::answer(modules::d02::p1());
            modules::commands::answer(modules::d02::p2());
        }
        "3" => {
            modules::commands::answer(modules::d03::p1());
            modules::commands::answer(modules::d03::p2());
        }
        _ => {}
    }
}
