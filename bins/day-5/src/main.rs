#![allow(non_snake_case, non_upper_case_globals)]

use regex::Regex;
use get_input::{getInput};

static TestInput: &'static str = 
"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[derive(Debug)]
struct Move {
    amount: u32,
    from: usize,
    to: usize,
}

struct State {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

fn parseInput(input: &String) -> State {
    let stackRegex = Regex::new(r"\s{3} |\[(\w)\]").unwrap();
    let moveRegex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut lines: Vec<&str> = input.lines().rev().collect();
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut nextLine = lines.pop().unwrap();    
    while !nextLine.contains(&"1") {
        for (i, item) in stackRegex.captures_iter(nextLine).enumerate() {
            let stack = match stacks.get_mut(i) {
                Some(stack) => stack,
                None => {
                    let stack = Vec::new();
                    stacks.insert(i, stack);
                    stacks.get_mut(i).unwrap()
                }
            };

            let result = match item.get(1) {
                Some(item) => Some(item.as_str().chars().collect::<Vec<char>>()[0]),
                None => None,
            };

            if !result.is_none() {
                stack.push(result.unwrap());
            }
        }

        nextLine = lines.pop().unwrap();
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let mut moves: Vec<Move> = Vec::new();    
    lines.pop().unwrap();
    lines.reverse();
    for line in lines {
        let caps = moveRegex.captures(line).expect(format!("Failed {}", line).as_str());

        let craneMove = Move {
            amount: caps.get(1).unwrap().as_str().parse().unwrap(),
            from: caps.get(2).unwrap().as_str().parse().unwrap(),
            to: caps.get(3).unwrap().as_str().parse().unwrap(),
        };
        
        moves.push(craneMove);
    }

    return State {
        stacks,
        moves,
    };

}

#[derive(PartialEq, Eq)]
enum CraneType {
    Reverser,
    Normal,
}

fn moveCrates(state: &mut State, craneType: &CraneType) -> String {
    for (_, craneMove) in state.moves.iter().enumerate() {
        let mut temp: Vec<char> = Vec::new();
        {
            let fromStack = state.stacks.get_mut(craneMove.from - 1).unwrap();
            for _ in 0..craneMove.amount {
                temp.push(fromStack.pop().unwrap());
            }
        }
        if craneType == &CraneType::Reverser {
            temp.reverse();
        }

        let toStack = state.stacks.get_mut(craneMove.to - 1).unwrap();
        for item in temp {
            toStack.push(item);
        }
    }

    return state.stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>();
}

fn main() {
    let input = getInput(&2022,&5, TestInput);
    let mut state = parseInput(&input);

    println!("Part one: {}", moveCrates(&mut state, &CraneType::Normal));    

    state = parseInput(&input);
    println!("Part two: {}", moveCrates(&mut state, &CraneType::Reverser));
}
