use crate::basics::{input, lines};

pub fn p1() -> u32 {
    let mut count = 0;
    for line in lines(input(22, 4)) {
        let line = line.expect("Expected To Read Line");
        let elves: Vec<&str> = line.split(',').collect();
        let first: Vec<u8> = elves[0].split('-').map(|i| i.parse().unwrap()).collect();
        let second: Vec<u8> = elves[1].split('-').map(|i| i.parse().unwrap()).collect();
        if first[0] >= second[0] && first[1] <= second[1]
            || first[0] <= second[0] && first[1] >= second[1]
        {
            count += 1;
        }
    }
    count
}

pub fn p2() -> u32 {
    let mut count = 0;
    for line in lines(input(22, 4)) {
        let line = line.expect("Expected To Read Line");
        let elves: Vec<&str> = line.split(',').collect();
        let first: Vec<u8> = elves[0].split('-').map(|i| i.parse().unwrap()).collect();
        let second: Vec<u8> = elves[1].split('-').map(|i| i.parse().unwrap()).collect();
        if first[1] <= second[1] && first[1] >= second[0]
            || first[0] <= second[1] && first[0] >= second[0]
            || second[1] <= first[1] && second[1] >= first[0]
            || second[0] <= first[1] && second[0] >= first[0]
        {
            count += 1;
        }
    }
    count
}
