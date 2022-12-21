use crate::basics::{input, lines};
use std::path::PathBuf;

struct Directory(PathBuf, u32);

fn tree() -> Vec<Directory> {
    let mut directories: Vec<Directory> = Vec::new();
    let mut pwd = PathBuf::new();
    let mut size: u32 = 0;
    let mut sizes: Vec<u32> = Vec::new();

    for line in lines(input(22, 7)) {
        let line = line.expect("Expected To Read Line");
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[0] {
            "$" => match tokens[1] {
                "cd" => match tokens[2] {
                    ".." => {
                        directories.push(Directory(pwd.to_owned(), size));
                        size += sizes.pop().unwrap();
                        pwd.pop();
                    }
                    _ => {
                        sizes.push(size);
                        size = 0;
                        pwd.push(tokens[2]);
                    }
                },
                "ls" => {}
                _ => {}
            },
            "dir" => {}
            _ => size += tokens[0].parse::<u32>().unwrap(),
        }
    }

    directories.push(Directory(pwd.to_owned(), size));
    size += sizes.pop().unwrap();
    pwd.pop();
    directories.push(Directory(pwd.to_owned(), size));

    directories
}

pub fn p1() -> String {
    tree()
        .iter()
        .filter_map(|i| if i.1 <= 100_000 { Some(i.1) } else { None })
        .sum::<u32>()
        .to_string()
}

pub fn p2() -> String {
    let tree = tree();
    let target = 30_000_000 - (70_000_000 - tree.last().unwrap().1);
    tree.iter()
        .filter_map(|i| if i.1 >= target { Some(i.1) } else { None })
        .min()
        .unwrap()
        .to_string()
}
