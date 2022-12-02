use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut elves_calories: Vec<u32> = Vec::new();
    let mut total_for_elf: u32 = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(value) = line {
                if value.is_empty() && total_for_elf > 0{
                    elves_calories.push(total_for_elf);
                    total_for_elf = 0;
                } else {
                    total_for_elf += value.parse::<u32>().unwrap();
                }
            }
        }
    }
    println!("### list of elves calories ###");
    println!("{:?}", elves_calories);

    println!("### max calories ###");
    println!("{:?}", elves_calories.iter().max().unwrap());

    println!("### list of top three elves calories ###");
    let mut sorted_vec = elves_calories.clone();
    sorted_vec.sort();
    let top_three_calories = &sorted_vec[sorted_vec.len()-3..sorted_vec.len()];
    println!("{:?}", top_three_calories);

    println!("### sum of top three elves calories ###");
    println!("{:?}", top_three_calories.iter().sum::<u32>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}