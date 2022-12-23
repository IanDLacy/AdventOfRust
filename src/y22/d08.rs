use crate::basics::{input, lines};

struct Tree {
    height: u8,
    visible: bool,
    score: u32,
}

fn rows() -> Vec<Vec<Tree>> {
    let mut rows: Vec<Vec<Tree>> = Vec::new();
    for (row, line) in lines(input(22, 8)).enumerate() {
        rows.push(Vec::new());
        let line = line.expect("Expected To Read Line");
        for char in line.chars() {
            rows[row].push(Tree {
                height: char.to_digit(10).unwrap() as u8,
                visible: false,
                score: 1,
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
                height: tree.height,
                visible: tree.visible,
                score: tree.score,
            });
        }
    }
    cols
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

fn scan(trees: Vec<Tree>) -> Vec<Tree> {
    let mut scanned: Vec<Tree> = Vec::new();
    let mut history: [usize; 10] = [0; 10];
    for (index, tree) in trees.iter().enumerate() {
        let mut pushee = Tree {
            height: tree.height,
            visible: tree.visible,
            score: tree.score,
        };
        let shift: usize;
        if history[tree.height as usize] == 0 {
            pushee.visible = true;
            shift = 0;
        } else {
            shift = 1;
        }
        pushee.score *= (index + shift - history[tree.height as usize]) as u32;
        for (height, _) in history.into_iter().enumerate() {
            if height <= tree.height as usize {
                history[height] = index + 1;
            }
        }
        scanned.push(pushee);
    }
    scanned
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
    look(cols(look(rows())))
        .iter()
        .flatten()
        .map(|i| i.score)
        .max()
        .unwrap()
        .to_string()
}
