#![allow(non_upper_case_globals, non_snake_case)]
use get_input;

static TestInput: &'static str = 
"vJrwpWtwJgWr hcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";


static CharValues: &'static [char] = &['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w','x','y','z','A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q','R','S','T','U','V','W','X','Y','Z'];
fn getValue(character: &char) -> u32 {
    return TryInto::<u32>::try_into(CharValues.iter().position(|x| x == character).unwrap()).unwrap() + 1u32;
}

struct Rucksack(Vec<char>, Vec<char>);

// fn rucksackToString(rs: &Rucksack) -> String {
//     return format!("{} | {}", 
//         String::from_iter(rs.0.iter()), 
//         String::from_iter(rs.1.iter())
//     )
// }

fn joinRucksack(rs: &Rucksack) -> String {
    let s1 = rs.0.iter().collect::<String>();
    let mut s2 = rs.1.iter().collect::<String>();
    s2.push_str(&s1);
    
    return s2;
}

fn parseInput(input: &String) -> Vec<Rucksack> {
    let mut result: Vec<Rucksack> = Vec::new();
    for line in input.lines() {
        let rs = Rucksack(Vec::from_iter(line[..line.len() /2].chars()), Vec::from_iter(line[line.len()/2..].chars()));
        result.push(rs);
    }

    return result;
}

fn parseGroups(rucksacks: &Vec<Rucksack>) -> Vec<[&Rucksack; 3]> {
    let mut i = 0;
    let mut groups: Vec<[&Rucksack; 3]> = Vec::new();
    while i < rucksacks.len() - 1 {
        groups.push([&rucksacks[i], &rucksacks[i + 1], &rucksacks[i + 2]]);
        i += 3;
    }
    return groups;
}

fn findRepeatedChar(rucksack: &Rucksack) -> char {
    for leftChar in rucksack.1.iter() {
        for rightChar in rucksack.0.iter() {
            if leftChar == rightChar {
                return leftChar.clone();
            }
        }
    }

    panic!("Did not find repeated char!");
}

fn partOne(rucksacks: &Vec<Rucksack>) -> u32 {
    let mut sum = 0u32;
    for rs in rucksacks.iter() {
        sum += getValue(&findRepeatedChar(rs));
    } 

    return sum;
}

fn findCommonChar(group: &[&Rucksack; 3]) -> char {
    for elfOne in joinRucksack(group[0]).chars() {
        for elfTwo in joinRucksack(group[1]).chars() {
            if elfOne == elfTwo {
                for elfThree in joinRucksack(group[2]).chars() {
                    if elfOne == elfThree {
                        return elfOne;
                    }
                }
            }
        }
    }

    panic!("No badge item found");
}

fn partTwo(rucksacks: &Vec<Rucksack>) -> u32 {
    let groups = parseGroups(rucksacks);

    let mut sum = 0u32;
    for group in groups.iter() {
        sum += getValue(&findCommonChar(group));
    }
    
    return sum;
}

fn main() {
    let input = get_input::getInput(&2022,&3, TestInput);

    let rucksacks = parseInput(&input);

    // println!("parsed:\n{}", rucksacks.iter().map(|rs| rucksackToString(&rs)).collect::<Vec<String>>().join("\n"));

    println!("Part one: {}", partOne(&rucksacks));

    println!("Part two: {}", partTwo(&rucksacks));
    
    
}
