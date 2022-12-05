#![allow(non_snake_case)]
use ureq;
use std::env;
use std::fs::{File, create_dir_all};
use std::io::{Read, Write};

fn useTestInput() -> bool {
    let args = env::args().collect::<Vec<String>>();
    return args.contains(&String::from("test"));
}

pub fn getInput(year: &u32, day: &u32, testInput: &str) -> String {
    if useTestInput() {
        return String::from(testInput);
    }

    let mut path = env::temp_dir();
    path.push(format!("aoc-input\\{}\\{}\\input.txt", year, day));

    if path.exists() {
        let mut contents = String::new();
        let mut file = File::open(path).unwrap();
        file.read_to_string(&mut contents).unwrap();
        return contents;
    }


    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let res: ureq::Response = ureq::get(&url)
        .set("Cookie", "session=53616c7465645f5f512cbc78ccfeca9a9810a03d8151f22d2fc5ea5e53a3572267c255b9cbe2df0876afb15040f2d75b13ec95cbebf3abb162228c8d81d11713")
        .call().unwrap();

    if res.status() == 200 {
        let contents = res.into_string().unwrap();
        create_dir_all(path.parent().unwrap()).unwrap();
        let mut file = File::create(path).unwrap();
        file.write_all(&contents.as_bytes()).unwrap();
        return contents;
    }

    panic!("Failed to get input!\nstatus: {}\nbody={}", res.status(), res.into_string().unwrap());
}
