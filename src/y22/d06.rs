use crate::basics::{input, lines};
use std::collections::VecDeque;

pub fn p1() -> String {
    let mut answer = 0;
    for line in lines(input(22, 6)) {
        let line = line.expect("Expected To Read Line");
        let mut queue = VecDeque::new();
        let mut unique: u16 = 0;
        for (index, char) in line.chars().enumerate() {
            if index > 3 {
                queue.pop_front();
            }
            if queue.contains(&char) {
                unique = 0;
            } else {
                unique += 1;
            }
            queue.push_back(char);
            if unique == 4 {
                answer = index;
                break;
            }
        }
    }
    answer.to_string()
}

pub fn p2() -> String {
    let mut answer = 0;
    for line in lines(input(22, 6)) {
        let line = line.expect("Expected To Read Line");
        let mut queue = VecDeque::new();
        for (index, char) in line.chars().enumerate() {
            if queue.len() == 14 {
                let mut unique = 0;
                for item in &queue {
                    let mut count = 0;
                    for sibling in &queue {
                        if item == sibling {
                            count += 1;
                        }
                    }
                    if count == 1 {
                        unique += 1;
                    }
                }
                if unique == 14 {
                    answer = index;
                    break;
                }
                queue.pop_front();
            }
            queue.push_back(char);
        }
    }
    answer.to_string()
}
