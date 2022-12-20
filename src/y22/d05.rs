use crate::basics::{input, lines};

pub fn p1() -> String {
    let mut stacks = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let mut init: Vec<String> = lines(input(22, 5))
        .take(8)
        .map(|i| i.expect("Expected To Read Line"))
        .collect();
    init.reverse();
    for line in init {
        for (stack, char) in line.chars().skip(1).step_by(4).enumerate() {
            if char != ' ' {
                stacks[stack].push(char);
            }
        }
    }

    for line in lines(input(22, 5)).skip(10) {
        let line = line.expect("Expected To Read Line");
        let instruction: Vec<usize> = line
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|i| i.parse().unwrap())
            .collect();
        for _ in 0..instruction[0] {
            let popped = stacks[instruction[1] - 1].pop().unwrap();
            stacks[instruction[2] - 1].push(popped);
        }
    }

    let mut answer = String::new();
    for stack in stacks {
        answer.push(stack.last().unwrap().to_owned());
    }
    answer
}

pub fn p2() -> String {
    let mut stacks = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    let mut init: Vec<String> = lines(input(22, 5))
        .take(8)
        .map(|i| i.expect("Expected To Read Line"))
        .collect();
    init.reverse();
    for line in init {
        for (stack, char) in line.chars().skip(1).step_by(4).enumerate() {
            if char != ' ' {
                stacks[stack].push(char);
            }
        }
    }

    for line in lines(input(22, 5)).skip(10) {
        let line = line.expect("Expected To Read Line");
        let instruction: Vec<usize> = line
            .split(' ')
            .skip(1)
            .step_by(2)
            .map(|i| i.parse().unwrap())
            .collect();
        let mut popped = Vec::new();
        for _ in 0..instruction[0] {
            popped.push(stacks[instruction[1] - 1].pop().unwrap());
        }
        for _ in 0..instruction[0] {
            stacks[instruction[2] - 1].push(popped.pop().unwrap());
        }
    }

    let mut answer = String::new();
    for stack in stacks {
        answer.push(stack.last().unwrap().to_owned());
    }
    answer
}
