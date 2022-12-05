use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut fully_contained = 0;
    let mut just_overlapping = 0;

    if let Ok(lines) = read_lines("./sections.txt") {
        for line in lines {
            if let Ok(value) = line {
                let converted =  value.replace(",", "-");
                let ranges: Vec<u32> = converted.split("-").into_iter().map(|number| number.parse::<u32>().unwrap()).collect();
                dbg!(&ranges);
                let a = ranges[0]..=ranges[1];
                let b = ranges[2]..=ranges[3];
                // fully contained
                if (a.contains(&ranges[2]) && a.contains(&ranges[3])) | (b.contains(&ranges[0]) && b.contains(&ranges[1])) {
                    fully_contained += 1;
                }
                // just overlapping
                if a.contains(&ranges[2]) | a.contains(&ranges[3]) | b.contains(&ranges[0]) | b.contains(&ranges[1]){
                    just_overlapping +=1;
                }
            }
        }
    }
    dbg!(fully_contained);
    dbg!(just_overlapping);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}