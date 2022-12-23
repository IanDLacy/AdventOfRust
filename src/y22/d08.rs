use crate::basics::{input, lines};

struct Tree {
    visible: bool,
    height: u8,
}

fn rows() -> Vec<Vec<Tree>> {
    let mut rows: Vec<Vec<Tree>> = Vec::new();
    for (row, line) in lines(input(22, 8)).enumerate() {
        rows.push(Vec::new());
        let line = line.expect("Expected To Read Line");
        for char in line.chars() {
            rows[row].push(Tree {
                visible: false,
                height: char.to_digit(10).unwrap() as u8,
            })
        }
    }
    rows
}

fn cols(rows: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    let mut cols: Vec<Vec<Tree>> = Vec::new();
    for row in rows {
        for (col, tree) in row.iter().enumerate() {
            if col + 1 > cols.len() {
                cols.push(Vec::new());
            }
            cols[col].push(Tree {
                visible: tree.visible,
                height: tree.height,
            });
        }
    }
    cols
}

fn scan(trees: Vec<Tree>) -> Vec<Tree> {
    let mut scanned: Vec<Tree> = Vec::new();
    let mut max: u8 = 0;
    for tree in trees {
        if tree.height + 1 > max {
            max = tree.height + 1;
            scanned.push(Tree {
                visible: true,
                height: tree.height,
            });
        } else {
            scanned.push(Tree {
                visible: tree.visible,
                height: tree.height,
            });
        }
    }
    scanned
}

fn look(vec: Vec<Vec<Tree>>) -> Vec<Vec<Tree>> {
    let mut looked: Vec<Vec<Tree>> = Vec::new();
    for mut item in vec {
        item = scan(item);
        item.reverse();
        item = scan(item);
        looked.push(item);
    }
    looked
}

pub fn p1() -> String {
    look(cols(look(rows())))
        .iter()
        .flatten()
        .filter(|i| i.visible)
        .count()
        .to_string()
}

pub fn p2() -> String {
    for line in lines(input(22, 8)) {
        line.expect("Expected To Read Line");
    }
    "answer".to_owned()
}
