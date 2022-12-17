use crate::basics::*;

pub fn p1() -> u32 {
    let mut score: u32 = 0;
    for line in lines(path(2)) {
        let line = line.expect("Expected To Read Line");
        let mut chars = line.chars();
        score += match (chars.next().unwrap(), chars.last().unwrap()) {
            ('B', 'X') => 0 + 1,
            ('C', 'Y') => 0 + 2,
            ('A', 'Z') => 0 + 3,
            ('A', 'X') => 3 + 1,
            ('B', 'Y') => 3 + 2,
            ('C', 'Z') => 3 + 3,
            ('C', 'X') => 6 + 1,
            ('A', 'Y') => 6 + 2,
            ('B', 'Z') => 6 + 3,
            _ => 0,
        }
    }
    score
}

pub fn p2() -> u32 {
    let mut score: u32 = 0;
    for line in lines(path(2)) {
        let line = line.expect("Expected To Read Line");
        let mut chars = line.chars();
        score += match (chars.next().unwrap(), chars.last().unwrap()) {
            ('B', 'X') => 0 + 1,
            ('C', 'X') => 0 + 2,
            ('A', 'X') => 0 + 3,
            ('A', 'Y') => 3 + 1,
            ('B', 'Y') => 3 + 2,
            ('C', 'Y') => 3 + 3,
            ('C', 'Z') => 6 + 1,
            ('A', 'Z') => 6 + 2,
            ('B', 'Z') => 6 + 3,
            _ => 0,
        }
    }
    score
}
