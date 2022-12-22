use crate::basics::{input, lines};

pub fn p1() -> String {
    for line in lines(input(22, 8)) {
        line.expect("Expected To Read Line");
    }
    "answer".to_owned()
}

pub fn p2() -> String {
    for line in lines(input(22, 8)) {
        line.expect("Expected To Read Line");
    }
    "answer".to_owned()
}
