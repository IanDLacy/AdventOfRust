use crate::basics::*;

const PATH: &str = "./input/01.txt";

fn get_elf(lines: &mut Lines<BufReader<File>>) -> i32 {
    let mut sum = 0;
    for line in lines {
        let line = line.expect("Expected To Read Line");
        if line == "" {
            break;
        }
        sum += line
            .parse::<i32>()
            .expect("Expected Integer Compatible String");
    }
    sum
}

pub fn p1() -> i32 {
    let mut lines = get_lines(PATH);
    let mut max = 0;
    let mut elf: i32;
    loop {
        elf = get_elf(&mut lines);
        if elf == 0 {
            break;
        }
        if elf > max {
            max = elf
        }
    }
    max
}

pub fn p2() -> i32 {
    let mut lines = get_lines(PATH);
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut elf: i32;

    loop {
        elf = get_elf(&mut lines);
        if elf == 0 {
            break;
        }
        if elf >= first {
            third = second;
            second = first;
            first = elf;
        } else if elf >= second {
            third = second;
            second = elf;
        } else if elf >= third {
            third = elf;
        }
    }

    first + second + third
}
