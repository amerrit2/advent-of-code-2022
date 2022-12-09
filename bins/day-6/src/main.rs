#![allow(non_snake_case, non_upper_case_globals)]

use core::panic;
use std::{collections::{HashSet, HashMap, VecDeque}, num};

use get_input::{getInput, useTestInput};

static TestInputs: &'static [(&str, u32)] = &[
    ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
    ("nppdvjthqldpwncqszvftbrmjlhg", 6),
    ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
    ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
];

fn indexOfStartOfPacket(data: &str, numDistinct: usize) -> usize {
    let mut charMap: HashMap<char, u32> = HashMap::new();
    let mut deq: VecDeque<char> = VecDeque::new(); 

    for (index, ch) in data.chars().enumerate() {
        let newVal = match charMap.get_mut(&ch) {
            Some(val) => *val + 1,
            None => 1,
        };

        charMap.insert(ch, newVal);
        deq.push_back(ch);
        
        if deq.len() > numDistinct {
            let front = deq.pop_front().unwrap();
            
            let newVal = match charMap.get_mut(&front) {
                Some(val) => {
                    let newVal = *val - 1;
                    if newVal == 0 {
                        charMap.remove(&front);
                    }
                    newVal                    
                },
                None => panic!("Impossibru!"),
            };

            if newVal != 0 {
                charMap.insert(front, newVal);
            }
        }
        
        // println!("Iteration {index}:\ncharMap: {:?}\ndeq: {:?}\n\n", charMap, deq);
        if charMap.len() == numDistinct {
            return index + 1;
        }
    }

    panic!("Failed to find 4 repeating chars")
}

fn partOne(input: &str) {
    println!("Part one: {}", indexOfStartOfPacket(input, 4));
}

fn partTwo(input: &str) {
    println!("Part two: {}", indexOfStartOfPacket(input, 14));
}

fn main() {
    if useTestInput() {
        for (input, solution) in TestInputs.iter() {
            partOne(input);
            println!("Expected {solution}");
        }
    } else {
        let input = getInput(&2022,&6, "");
        partOne(&input);
        partTwo(&input)
    }
}
