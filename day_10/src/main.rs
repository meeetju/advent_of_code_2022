use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

const NO_OPERATION: &str = "noop";
const ADD: &str = "addx";
const CRITICAL_CYCLES: [i32; 6] = [20, 60, 100, 140, 180, 220]; 

fn main() {

    let mut registry: i32 = 1;
    let mut cycle: i32 = 0;
    let mut registry_values: Vec<i32> = vec![];
    let mut drawing: Vec<Vec<&str>> = vec![vec!["."; 40]; 6];


    if let Ok(lines) = read_lines("./instructions.txt") {
        for line in lines {
            if let Ok(value) = line {
                let new_move: Vec<&str> = value.split(" ").into_iter().collect();
                let instruction = new_move[0];

                match instruction {
                    // one cycle
                    NO_OPERATION => {
                        cycle += 1;
                        update_drawing(&mut drawing, cycle, registry);
                        store_registry(&mut registry_values, cycle, registry)
                    },
                    // two cycles
                    ADD => {
                        cycle += 1;
                        update_drawing(&mut drawing, cycle, registry);
                        store_registry(&mut registry_values, cycle, registry);
                        cycle += 1;
                        update_drawing(&mut drawing, cycle, registry);
                        store_registry(&mut registry_values, cycle, registry);
                        registry += new_move[1].to_string().parse::<i32>().unwrap();
                    },
                    _ => {panic!("Something went wrong!")}
                }
            }
        }

        let mut strenghts: Vec<i32> = vec![];
        for (index, cycle) in CRITICAL_CYCLES.iter().enumerate() {
            strenghts.push(cycle * registry_values[index]);
        }

        dbg!(strenghts.iter().sum::<i32>());
        for line in drawing.iter() {
            println!("{}", line.concat());
        }
    }

}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}

fn store_registry(registry_values: &mut Vec<i32>, cycle: i32, registry: i32) {
    if CRITICAL_CYCLES.contains(&cycle) {
        registry_values.push(registry);
    }
}

fn update_drawing(drawing: &mut Vec<Vec<&str>>, cycle: i32, registry: i32) {
    let line = (cycle - 1) / 40;
    let index = (cycle - 1) % 40;

    if ((registry - 1) <= index) && (index <= (registry + 1)) {
        drawing[line as usize][index as usize] = "#";
    }
}