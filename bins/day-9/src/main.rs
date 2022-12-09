#![allow(non_snake_case, non_upper_case_globals)]

use std::collections::HashSet;
use get_input::{getInput};

static TestInput: &'static str = 
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

static TestInput2: &'static str = 
"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

fn nextTPos(hPos: &(i32, i32), tPos: &(i32, i32)) -> (i32, i32) {
    let diff = (hPos.0 - tPos.0, hPos.1 - tPos.1);
    let absDist = ((diff.0.pow(2) + diff.1.pow(2)) as f32).sqrt();
    let mut change = (0, 0);
    if absDist >= 2.0 {
        change.0 = match diff.0  {
            2 => 1,
            -2 => -1,
            0 => 0,
            other => other,
        };
    
        change.1 = match diff.1 {
            2 => 1,
            -2 => -1,
            0 => 0,
            other => other,
        };
    }

    return (tPos.0 + change.0, tPos.1 + change.1);
    
}

fn parseInput(input: &String) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut hPos: (i32, i32) = (0, 0);
    let mut tPos: (i32, i32) = (0, 0);

    visited.insert(tPos.clone());

    for line in input.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        let times: u32 = tokens.get(1).unwrap().parse().unwrap();
        let dir = tokens.get(0).unwrap();

        for _ in 0..times {
            match dir {
                &"R" => {
                    hPos.0 += 1;
                },
                &"U" => {
                    hPos.1 += 1;
                },
                &"D" => {
                    hPos.1 -= 1;
                },
                &"L" => {
                    hPos.0 -= 1;
                },
                _ => panic!("Unrecognized direction {dir}"),
            }
            
            let newTPos = nextTPos(&hPos, &tPos);
            visited.insert(newTPos.clone());
            tPos = newTPos;
        }
    }

    return visited.len();

}


fn parseInput2(input: &String) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut knots: Vec<(i32, i32)> = Vec::new();
    for _ in 0..=9 {
        knots.push((0, 0));
    }

    visited.insert(knots.get(9).unwrap().clone());

    for line in input.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();
        let times: u32 = tokens.get(1).unwrap().parse().unwrap();
        let dir = tokens.get(0).unwrap();

        for _ in 0..times {
            let head = knots.get_mut(0).unwrap();
            match dir {
                &"R" => {
                    head.0 += 1;
                },
                &"U" => {
                    head.1 += 1;
                },
                &"D" => {
                    head.1 -= 1;
                },
                &"L" => {
                    head.0 -= 1;
                },
                _ => panic!("Unrecognized direction {dir}"),
            }

            for curIdx in 0..knots.len() - 1 {
                let hPos = knots.get(curIdx).unwrap().clone();
                let nextKnot = knots.get_mut(curIdx + 1).unwrap();
                let newPos = nextTPos(&hPos, nextKnot);
                nextKnot.0 = newPos.0;
                nextKnot.1 = newPos.1;
            }
            
            visited.insert(knots.last().unwrap().clone());
        }
    }

    return visited.len();

}


fn main() {
    let input = getInput(&2022, &9, TestInput);

    println!("Part one: {}", parseInput(&input));

    let input2 = getInput(&2022, &9, TestInput2);

    println!("Part two:{}", parseInput2(&input2));
}