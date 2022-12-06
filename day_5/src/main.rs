use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut stacks_section: bool = true;
    let crates_section_pattern = " 1   2   3   4   5   6   7   8   9 ";
    let mut stacks: Vec<Vec<String>> = vec![vec![]; 9];

    if let Ok(lines) = read_lines("./stack_crates.txt") {
        for line in lines {
            if let Ok(value) = line {
                if stacks_section {
                    if value == crates_section_pattern {
                        stacks_section = false;
                        continue;
                    }
                    for (i, _) in value.chars().enumerate().step_by(4) {
                        let stack_crate = &value[i+1..i+2].to_string().clone();
                        let stack_index = i/4;
                        if stack_crate != " " {
                            stacks[stack_index].insert(0, stack_crate.clone());
                        }
                    }
                } else {
                    if !value.is_empty() {
                        let numbers: Vec<u32> = value.split(" ").filter(|value| value.parse::<u32>().is_ok()).map(|value| value.parse::<u32>().unwrap()).collect();

                        // one crate at a time crane-9000
                        // for _ in 0..numbers[0] {
                        //     let moved_crate = stacks[numbers[1] as usize - 1].pop().unwrap();
                        //     stacks[numbers[2] as usize - 1].push(moved_crate);
                        // }

                        // multiple crates at a time crane-9001
                        let mut moved_crates: Vec<String> = vec![];
                        for _ in 0..numbers[0] {
                            moved_crates.push(stacks[numbers[1] as usize - 1].pop().unwrap());
                        }
                        moved_crates.reverse();
                        stacks[numbers[2] as usize - 1].append(&mut moved_crates);
                    }
                }
            }
        }
        println!("{}", "And the result is...");
        for stack in stacks {
            print!("{}", stack.last().unwrap());
        }

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}