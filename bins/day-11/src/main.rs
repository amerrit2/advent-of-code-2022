#![allow(non_snake_case, non_upper_case_globals)]
use std::collections::VecDeque;
use regex::{ Regex, Captures };
use get_input::getInput;
use core::fmt::Display;
use num::BigUint;

static TestInput: &'static str =
    "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

type Item = BigUint;
type MonkeyId = usize;

struct Monkey {
    items: VecDeque<Item>,
    itemUpdater: Box<dyn Fn(Item) -> Item>,
    nextMonkey: Box<dyn Fn(Item) -> MonkeyId>,
    inspectionCount: usize,
}

struct Troop {
    monkeys: Vec<Monkey>,
}

impl Troop {
    fn new() -> Self {
        Self {
            monkeys: Vec::new(),
        }
    }

    fn executeRounds(&mut self, num: u32) {
        for i in 0..num {
            if i % 100 == 0 {
                println!("Round {i}");
            }
            
            for id in 0..self.monkeys.len() {
                let monkey = self.monkeys.get_mut(id).unwrap();
                let numItems = monkey.items.len();
                monkey.inspectionCount += numItems;
                let mut changes: Vec<(MonkeyId, Item)> = Vec::new();
                for _ in 0..numItems {
                    let item = monkey.items.pop_front().unwrap();
                    let newItem = (monkey.itemUpdater)(item);
                    let nextMonkeyId = (monkey.nextMonkey)(newItem.clone());
                    changes.push((nextMonkeyId, newItem));
                }

                for (id, item) in changes {
                    self.monkeys.get_mut(id).unwrap().items.push_back(item);
                }
            }
        }
    }

    fn monkeyBusiness(&self) -> usize {
        let mut counts = self.monkeys
            .iter()
            .map(|mon| mon.inspectionCount)
            .collect::<Vec<usize>>();
        counts.sort();
        return counts.last().unwrap() * counts.get(counts.len() - 2).unwrap();
    }
}

impl Display for Troop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (id, mon) in self.monkeys.iter().enumerate() {
            f.write_str(&format!("({}: [", id)).unwrap();
            f.write_str(
                mon.items
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
                    .as_str()
            ).unwrap();
            f.write_str("]) ").unwrap();
        }

        return f.write_str("");
    }
}

fn makeItemUpdater(captures: &Captures, divideByThree: bool) -> Box<dyn Fn(Item) -> Item> {
    let op = captures.get(1).unwrap().as_str();
    let val: Result<BigUint, _> = captures.get(2).unwrap().as_str().parse();
    match op {
        "*" =>
            Box::new(
                move |old|
                    (old.clone() *
                        (match &val {
                            Ok(v) => v.clone(),
                            _ => old.clone(),
                        })) /
                    BigUint::from(match divideByThree {
                        true => 3u32,
                        false => 1u32,
                    })
            ),
        "+" =>
            Box::new(
                move |old|
                    (old.clone() +
                        (match &val {
                            Ok(v) => v.clone(),
                            _ => old.clone(),
                        })) /
                    BigUint::from(match divideByThree {
                        true => 3u32,
                        false => 1u32,
                    })
            ),
        _ => panic!("Invalid op: {op}"),
    }
}

fn makeNextMonkey<'a>(divisibleTest: usize, trueMonkey: usize, falseMonkey: usize) -> Box<dyn Fn(Item) -> MonkeyId> {
    Box::new(move |item| {
        if item % BigUint::from(divisibleTest) == BigUint::from(0u32) {
            return trueMonkey;
        }

        return falseMonkey;
    })
}

fn parseInput(input: &str, divideByThree: bool) -> Troop {
    let mut troop = Troop::new();

    let monIdRegex = Regex::new(r"Monkey (\d+):").unwrap();
    let itemRegex = Regex::new(r"(\d+),?\s?").unwrap();
    let opRegex = Regex::new(r"new = old (\+|\*) (\d+|old)").unwrap();
    let testRegex = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let trueFalseRegex = Regex::new(r"(\d+)").unwrap();

    let mut lineIter = input.lines();
    while let Some(line) = lineIter.next() {
        println!("Line: '{line}'");
        let monId: usize = monIdRegex.captures(line).unwrap().get(1).unwrap().as_str().parse().unwrap();

        let mut items: VecDeque<Item> = VecDeque::new();
        for item in itemRegex.captures_iter(lineIter.next().unwrap()) {
            items.push_back(item.get(1).unwrap().as_str().parse().unwrap());
        }

        let opCaptures = opRegex.captures(lineIter.next().unwrap()).unwrap();
        let itemUpdater = makeItemUpdater(&opCaptures, divideByThree);

        let divisibleTest: usize = testRegex
            .captures(lineIter.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let trueMonkey: usize = trueFalseRegex
            .captures(lineIter.next().unwrap())
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse()
            .unwrap();

        let falseMonkey: usize = trueFalseRegex
            .captures(lineIter.next().unwrap())
            .unwrap()
            .get(1)

            .unwrap()
            .as_str()
            .parse()
            .unwrap();
        troop.monkeys.insert(monId, Monkey {
            items,
            itemUpdater,
            nextMonkey: makeNextMonkey(divisibleTest, trueMonkey, falseMonkey),
            inspectionCount: 0,
        });
        lineIter.next();
    }

    return troop;
}

fn main() {
    let input = getInput(&2022, &11, TestInput);
    let mut troop = parseInput(&input, true);
    troop.executeRounds(20);
    println!("Part one: {}", troop.monkeyBusiness());

    let mut troop2 = parseInput(&input, false);
    troop2.executeRounds(10000);
    println!("Part two: {}", troop.monkeyBusiness());
}