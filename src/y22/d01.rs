use crate::basics::*;

fn elf(lines: &mut Lines<BufReader<File>>) -> u32 {
    let mut sum = 0;
    for line in lines {
        let line = line.expect("Expected To Read Line");
        if line == "" {
            break;
        }
        sum += line
            .parse::<u32>()
            .expect("Expected Integer Compatible String");
    }
    sum
}

pub fn p1() -> String {
    let mut lines = lines(input(22, 1));
    let mut max = 0;
    let mut sum;
    loop {
        sum = elf(&mut lines);
        if sum == 0 {
            break max.to_string();
        }
        if sum > max {
            max = sum;
        }
    }
}

pub fn p2() -> String {
    let mut lines = lines(input(22, 1));
    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut sum;

    loop {
        sum = elf(&mut lines);
        if sum == 0 {
            break (first + second + third).to_string();
        }
        if sum >= first {
            third = second;
            second = first;
            first = sum;
        } else if sum >= second {
            third = second;
            second = sum;
        } else if sum >= third {
            third = sum;
        }
    }
}
