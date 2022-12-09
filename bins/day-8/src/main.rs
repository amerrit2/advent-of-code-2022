#![allow(non_upper_case_globals, non_snake_case)]
use std::fmt::Write;

use get_input::{getInput};

static TestInput: &'static str = 
"30373
25512
65332
33549
35390";

#[derive(Debug)]
struct Tree {
    height: u32,
    isVisible: bool,
    leftScore: u32,
    rightScore: u32,
    upScore: u32,
    downScore: u32,
}

fn parseInput(input: &str) -> Vec<Vec<Tree>> {
    let mut out: Vec<Vec<Tree>> = Vec::new();
    let numRows = input.lines().count();
    let numCols = input.lines().next().unwrap().chars().count();
    for (rowIdx, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (colIdx, ch) in line.chars().enumerate() {
            let height = ch.to_digit(10).unwrap();
            row.push(Tree {
                height,
                isVisible:  rowIdx == 0 || (rowIdx == numRows - 1) || colIdx == 0 || (colIdx == numCols - 1),
                upScore: 0,
                downScore: 0,
                leftScore: 0,
                rightScore: 0,
            });
        }
        out.push(row);
    }
    return out;
}

fn processRowsRightDown(trees: &mut Vec<Vec<Tree>>) {
    let mut columnMaxes: Vec<u32> = Vec::new();
    for row in trees {
        let mut rowMax = 0;
        for (colIdx, tree) in row.iter_mut().enumerate() {
            match columnMaxes.get_mut(colIdx) {
                Some(height) => {
                    if tree.height > *height {
                        *height = tree.height;
                        tree.isVisible = true;
                    }
                }, 
                None => {
                    columnMaxes.insert(colIdx, tree.height);
                    tree.isVisible = true;
                }            
            }
            if tree.height > rowMax {
                rowMax = tree.height;
                tree.isVisible = true;
            }
        }
    }
}

fn processRowsLeftUp(trees: &mut Vec<Vec<Tree>>) {
    let mut columnMaxes: Vec<u32> = Vec::new();
    for row in trees.iter_mut().rev() {
        let mut rowMax = 0;
        for (colIdx, tree) in row.iter_mut().rev().enumerate() {
            match columnMaxes.get_mut(colIdx) {
                Some(height) => {
                    if tree.height > *height {
                        *height = tree.height;
                        tree.isVisible = true;
                    }
                }, 
                None => {
                    columnMaxes.insert(colIdx, tree.height);
                    tree.isVisible = true;
                }            
            }
            if tree.height > rowMax {
                rowMax = tree.height;
                tree.isVisible = true;
            }
        }
    }
}

fn countVisible(trees: &Vec<Vec<Tree>>) -> u32 {
    let mut count = 0u32;
    for row in trees {
        let mut line = String::from("");
        for tree in row {
            if tree.isVisible {
                count += 1;
                line.write_str("1").unwrap();
            } else {
                line.write_str("0").unwrap();
            }
        }
        // println!("{line}");
    }

    return count;
}

fn scenicScoresRightDown(trees: &mut Vec<Vec<Tree>>) {
    let mut seenCols: Vec<Vec<&Tree>> = Vec::new();
    for row in trees.iter_mut() {
        let mut seenRow: Vec<&Tree> = Vec::new();
        for (colIdx, tree) in row.iter_mut().enumerate() {            
            // Backwards count
            let mut countBack = 0;
            for otherTree in seenRow.iter().rev() {
                countBack += 1;
                if otherTree.height >= tree.height {
                    break;
                } 
            }
            tree.leftScore = countBack;

            // Upwards count
            let seenCol = match seenCols.get_mut(colIdx) {
                Some(col) => col,
                None => {
                    seenCols.insert(colIdx, Vec::new());
                    seenCols.get_mut(colIdx).unwrap()
                }
            };

            let mut countUp = 0;
            for otherTree in seenCol.iter().rev() {
                countUp += 1;
                if otherTree.height >= tree.height {
                    break;
                }
            }
            
            tree.upScore = countUp;
            seenRow.push(tree);
            seenCol.push(tree);
        }
    }
}

fn scenicScoresLeftUp(trees: &mut Vec<Vec<Tree>>) {
    let mut seenCols: Vec<Vec<&Tree>> = Vec::new();
    for row in trees.iter_mut().rev() {
        let mut seenRow: Vec<&Tree> = Vec::new();
        for (colIdx, tree) in row.iter_mut().rev().enumerate() {            
            // Forwards count
            let mut countForward = 0;
            for otherTree in seenRow.iter().rev() {
                countForward += 1;
                if otherTree.height >= tree.height {
                    break;
                } 
            }
            tree.rightScore = countForward;

            // Downwards count
            let seenCol = match seenCols.get_mut(colIdx) {
                Some(col) => col,
                None => {
                    seenCols.insert(colIdx, Vec::new());
                    seenCols.get_mut(colIdx).unwrap()
                }
            };

            let mut countDown = 0;
            for otherTree in seenCol.iter().rev() {
                countDown += 1;
                if otherTree.height >= tree.height {
                    break;
                }
            }
            
            tree.downScore = countDown;
            seenRow.push(tree);
            seenCol.push(tree);
        }
    }
}

fn highestScenicScore(trees: &Vec<Vec<Tree>>) -> u32 {
    let mut max = 0;
    for row in trees {
        for tree in row {
            let score = tree.upScore * tree.downScore * tree.leftScore * tree.rightScore;
            if score > max {
                max = score;
            }
        }
    }

    return max;
}


fn main() {
    let input = getInput(&2022, &8, TestInput);

    let mut trees = parseInput(&input);

    processRowsRightDown(&mut trees);
    processRowsLeftUp(&mut trees);
    println!("Part one: {}", countVisible(&trees));
    
    scenicScoresRightDown(&mut trees);
    scenicScoresLeftUp(&mut trees);

    println!("Part two {:?}", highestScenicScore(&trees));
}
