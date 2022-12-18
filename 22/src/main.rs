mod basics;
mod d01;
mod d02;
mod d03;

use crate::basics::*;
use std::env;

fn main() {
    match env::args().nth(1).unwrap_or_default().as_str() {
        "commit" => {
            commit();
        }
        "1" => {
            answer(d01::p1());
            answer(d01::p2());
        }
        "2" => {
            answer(d02::p1());
            answer(d02::p2());
        }
        "3" => {
            answer(d03::p1());
            answer(d03::p2());
        }
        _ => {}
    }
}
