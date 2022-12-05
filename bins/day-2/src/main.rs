#![allow(non_snake_case, non_upper_case_globals)]

use std::env;

static InputContents: &'static str = include_str!("./input.txt");
static TestContents: &'static str = include_str!("./test_input.txt");

fn ShapePoints(input: &str) -> u32 {
    match input {
        "X" | "A" => 1,
        "Y" | "B" => 2,
        "Z" | "C" => 3,
        _ => panic!(),
    }
}

fn useTestInput() -> bool {
    let args = env::args().collect::<Vec<String>>();
    return args.contains(&String::from("test"));
}

fn scoreRoundPart1(theirPlay: &str, myPlay: &str) -> u32 {
    let myPoints = ShapePoints(&myPlay);
    return match theirPlay {
        "A" => {
            match myPlay {
                "X" => 3,
                "Y" => 6,
                "Z" => 0,
                _ => panic!()
            }
        },
        "B" => {
            match myPlay {
                "X" => 0,
                "Y" => 3,
                "Z" => 6,
                _ => panic!()
            }
        },
        "C" => {
            match myPlay {
                "X" => 6,
                "Y" => 0,
                "Z" => 3,
                _ => panic!()
            }
        }
        _ => panic!()
    } + myPoints;
}

fn scoreRoundPart2(theirPlay: &str, theResult: &str) -> u32 {    
    match theResult {
        "X" => { // lose
            0 + match theirPlay {
                "A" => ShapePoints(&"C"),
                "B" => ShapePoints(&"A"),
                "C" => ShapePoints(&"B"),
                _ => panic!()
            }
        },
        "Y" => { // draw
            3 + match theirPlay {
                "A" => ShapePoints(&"A"),
                "B" => ShapePoints(&"B"),
                "C" => ShapePoints(&"C"),
                _ => panic!()
            }
        },
        "Z" => { // win
            6 + match theirPlay {
                "A" => ShapePoints(&"B"),
                "B" => ShapePoints(&"C"),
                "C" => ShapePoints(&"A"),
                _ => panic!()
            }
        }
        _ => panic!()
    }
}

fn main() {
    let inputLines = if useTestInput() { TestContents.lines() } else { InputContents.lines() };

    let mut sum = 0;
    for line in inputLines.clone() {
        let plays: Vec<&str> = line.split(' ').collect();
        sum += scoreRoundPart1(plays[0], plays[1]);
    }

    println!("Part 1: {}", sum);

    let mut sum2 = 0;
    for line in inputLines.clone() {
        let plays: Vec<&str> = line.split(' ').collect();
        sum2 += scoreRoundPart2(plays[0], plays[1]);
    }

    println!("Part 2: {}", sum2);

}
