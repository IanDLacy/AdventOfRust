use crate::basics::{lines, path};

pub fn p1() -> u32 {
    let mut sum: u32 = 0;
    for line in lines(path(22, 3)) {
        let line = line.expect("Expected To Read Line");
        let half = line.len() / 2;
        let mut set: u64 = 0;
        for char in line[..half].chars() {
            let code = char as u8;
            let priority = if code > 96 { code - 96 } else { code - 38 };
            set |= 1 << priority - 1;
        }
        for char in line[half..].chars() {
            let code = char as u8;
            let priority = if code > 96 { code - 96 } else { code - 38 };
            if set & 1 << priority - 1 != 0 {
                sum += priority as u32;
                break;
            }
        }
    }
    sum
}

pub fn p2() -> u32 {
    let mut sum: u32 = 0;
    let mut member: u8 = 1;
    let mut union: u64 = 0;
    for line in lines(path(22, 3)) {
        let line = line.expect("Expected To Read Line");
        let mut set: u64 = 0;
        for char in line.chars() {
            let code = char as u8;
            let priority = if code > 96 { code - 96 } else { code - 38 };
            let bit: u64 = 1 << priority - 1;
            set |= bit;
            if member == 3 && union & bit != 0 {
                sum += priority as u32;
                break;
            }
        }
        if member == 1 {
            union = set;
        }
        if member == 2 {
            union &= set;
        }
        member = if member < 3 { member + 1 } else { 1 };
    }
    sum
}
