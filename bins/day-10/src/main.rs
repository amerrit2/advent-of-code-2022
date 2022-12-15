#![allow(non_snake_case, non_upper_case_globals)]
use get_input::getInput;

static TestInput: &'static str = 
"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

fn parseInput(input: &str) -> Vec<isize> {
    let mut xRegs:  Vec<isize> = Vec::new();
    xRegs.push(1);

    for line in input.lines() {
        let tokens: Vec<&str> = line.split(' ').collect();               
        
        match tokens.get(0).unwrap() {
            &"addx" => {
                let val: isize = tokens.get(1).expect("Failed to find addx value").parse().unwrap();
                let last = xRegs.last().unwrap().clone();
                xRegs.push(last);
                xRegs.push(last);
                xRegs.push(last);
                xRegs.push(last + val);
            },
            &"noop" => {
                xRegs.push(xRegs.last().unwrap().clone());
                xRegs.push(xRegs.last().unwrap().clone());
            },
            invalid => panic!("Unrecognized command {invalid}"),
        };
    }

    return xRegs;
}

fn getCycleVal<'a>(xRegs: &'a Vec<isize>, cycleNum: &usize) -> &'a isize {
    return xRegs.get((cycleNum * 2).checked_sub(1).unwrap() as usize).expect(&format!("Invalid cycle number {cycleNum}"));
}

fn sumSignals(indices: Vec<usize>, xRegs: &Vec<isize>) -> isize {
    let mut sum = 0;
    for idx in indices.iter() {
        sum += getCycleVal(xRegs, idx) * *idx as isize;
    }

    return sum;
}

fn renderScreen(xRegs: &Vec<isize>) {
    let mut line = String::new();
    let numCycles: usize = 6 * 40;
    for cycleNum in 1..=numCycles {
        let drawPos = ((cycleNum - 1) % 40) as isize;
        if drawPos == 0 {
            println!("{line}");
            line = String::new();
        }

        let spritePos = getCycleVal(xRegs, &cycleNum);
        if spritePos - 1 <= drawPos && drawPos <= spritePos + 1 {
            line.push_str("#");   
        } else {
            line.push_str(".");
        }
    }

    println!("{line}");
}


fn main() {
    let input = getInput(&2022, &10, TestInput);

    let xRegs = parseInput(&input);

    println!("Part one: {}", sumSignals(vec![20, 60, 100, 140, 180, 220], &xRegs));
    println!("Part two:");
    renderScreen(&xRegs);
    
}