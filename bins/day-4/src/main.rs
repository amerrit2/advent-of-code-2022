#![allow(non_upper_case_globals ,non_snake_case)]
use get_input::{getInput};

static TestInput: &'static str = 
"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[derive(Debug)]
struct Range {
    lowBound: u32,
    highBound: u32,
}

fn parseRange(raw: &str) -> Range {
    let (start, end) = raw.split_once('-').unwrap();
    return Range {
        lowBound: start.parse::<u32>().unwrap(), 
        highBound: end.parse::<u32>().unwrap(),
    }
}

fn parseInput(input: &String) -> Vec<(Range, Range)> {
    let mut result = Vec::new();
    for line in input.lines() {
        let elves = line.split_once(',').unwrap();
        let pair = (parseRange(elves.0), parseRange(elves.1));
        result.push(pair);
    }

    return result;
}

fn fullyOverlaps(pair: &(Range, Range)) -> bool {
    if pair.0.lowBound < pair.1.lowBound {
        return pair.0.highBound >= pair.1.highBound;
    } else if pair.0.lowBound > pair.1.lowBound {
        return pair.0.highBound <= pair.1.highBound;
    } 

    return true;
}

fn hasOverlap(pair: &(Range, Range)) -> bool {
    if pair.0.lowBound < pair.1.lowBound {
        return pair.0.highBound >= pair.1.lowBound;
    } else if pair.0.lowBound > pair.1.lowBound {
        return pair.1.highBound >= pair.0.lowBound;
    }

    return true;
}

fn partOne(pairs: &Vec<(Range, Range)>) -> u32 {
    let mut count = 0u32;
    for pair in pairs.iter() {
        if fullyOverlaps(pair) {
            count += 1;
        }
    }

    return count;
}

fn partTwo(pairs: &Vec<(Range, Range)>) -> u32 {
    let mut count = 0u32;
    for pair in pairs.iter() {
        if hasOverlap(pair) {
            count += 1;
        }
    }

    return count;
}

fn main() {
    let input = getInput(&2022,&4, TestInput);
    let pairs = parseInput(&input);

    println!("Part one: {}", partOne(&pairs));
    println!("Part two: {}", partTwo(&pairs));

}
