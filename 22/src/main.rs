mod basics;
mod d01;
mod d02;

use std::env;

fn main() {
    match &env::args().nth(1).unwrap_or_default()[..] {
        "1" => {
            println!("{}, {}", d01::p1(), d01::p2());
        }
        "2" => {
            println!("{}, {}", d02::p1(), d02::p2());
        }
        _ => {}
    }
}
