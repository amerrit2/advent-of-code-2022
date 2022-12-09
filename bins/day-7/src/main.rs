#![allow(non_snake_case, non_upper_case_globals)]
mod filesystem;
use get_input::{getInput};
use filesystem::{FileSystem, Dir};

static TestInput: &'static str = 
"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

static TotalAvailable: &'static usize = &70000000;
static Required: &'static usize = &30000000;

fn parseInput(input: &String) -> FileSystem {
    let mut fs = FileSystem::new();

    for line in input.lines() {
        println!("Parsing line: {line}");
        let tokens: Vec<&str> = line.split(' ').collect();
        match tokens.first().expect(&format!("Empty line: '{line}'")) {
            &"$" => {
                match tokens.get(1).expect("Failed to find command") {
                    &"cd" => {
                        fs.changeDir(tokens.get(2).expect(&format!("Failed to find dir name for line {line}")));
                    },
                    &"ls" => println!("Doing nothing on ls"),
                    something => panic!("Unrecognized command: {something}")
                }
            },
            first => {
                match first {
                    &"dir" => {
                        fs.ensureDir(tokens.get(1).expect(&format!("Failed to get dir name from line {line}")));
                    },
                    sizeStr => {
                        let size: usize = sizeStr.parse().expect(&format!("Failed to parse size: {sizeStr}"));
                        let name = tokens.get(1).expect("Failed to get filename");

                        fs.ensureFile(name, size);
                    }
                }
            }
        }
    }

    return fs;
}


fn findDirToDeleteSize(fs: &FileSystem) -> usize {
    let rootSize = fs.getDirSize(&"/");
    let neededSpace: usize = Required - (TotalAvailable - rootSize);

    let mut sizes = fs.dirs.values().collect::<Vec<&Dir>>();
    sizes.sort_by_key(|dir| dir.size);

    for dir in sizes.iter() {
        if dir.size > neededSpace {
            return dir.size;
        }
    }

    panic!("Failed to find dir to delete!");

}
fn main() {
    let input = getInput(&2022, &7, TestInput);

    let fileSystem = parseInput(&input);

    let mut sum = 0usize;
    fileSystem.dirs.iter().for_each(|(_, dir)| {
        if dir.size <= 100000 {
            sum += dir.size;
        }
    }); 

    println!("Part one: {}", sum);
    println!("Part two: {}", findDirToDeleteSize(&fileSystem));
}
