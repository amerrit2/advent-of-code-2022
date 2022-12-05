#![allow(non_snake_case, non_upper_case_globals)]

use std::env;

static InputContents: &'static str = include_str!("./input.txt");
static TestContents: &'static str = include_str!("./test_input.txt");

fn useTestInput() -> bool {
    let args = env::args().collect::<Vec<String>>();

    return args.contains(&String::from("test"));
}

fn largestCalories(elves: &Vec<Vec<u32>>) -> u32 {
    let mut largest: u32 = 0;
    for elf in elves.iter() {
        let total: u32 = elf.iter().sum();
        if total > largest {
            largest = total;
        }
    }

    return largest;
}

fn main() {
    let inputLines = if useTestInput() { TestContents.lines() } else { InputContents.lines() };
   
    let mut elves: Vec<Vec<u32>> = Vec::new();
    let mut current_elf: Vec<u32> = Vec::new(); 
    for line in inputLines {
        if line == "" {
            elves.push(Vec::from(&current_elf[..]));
            current_elf.clear();
        } else {
            let val: u32 = line.parse().unwrap();
            current_elf.push(val)
        }
    }
    elves.push(Vec::from(&current_elf[..]));


    let mut output = String::new(); 
    for (i, elf) in elves.iter().enumerate() {
        let line = format!("Elf {i}: {}", elf.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" "));
        output.push_str(&line);
        output.push_str("\n");
    }

    
    
    println!("Largest: {}", largestCalories(&elves).to_string());

    let mut elfSums: Vec<u32> = elves.iter().map(|elf| elf.iter().sum()).collect::<Vec<u32>>();
    elfSums.sort();


    println!("Top 3 sum: {}", &elfSums[elfSums.len() - 3..].iter().sum::<u32>());

}