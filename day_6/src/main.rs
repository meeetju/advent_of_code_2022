use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {

    let mut marker: Vec<char> = vec![];

    if let Ok(lines) = read_lines("./communicates.txt") {
        for line in lines {
            if let Ok(value) = line {
                println!("{}", value);
                for (i, c) in value.chars().enumerate() {
                    if marker.len() == 4{
                        marker.pop();
                        marker.insert(0, c);

                        let mut hash_map: HashMap<char, char> = HashMap::new();
                        let mut duplicate_found: bool = false;
                        for ch in marker.iter() {
                            match hash_map.insert(*ch, *ch) {
                                None => {},
                                Some(_) => {duplicate_found = true}
                            }
                        }
                        if !duplicate_found {
                            println!("Marker character: {}", i+1);
                            break;
                        }
                    } else {
                        marker.insert(0, c);
                    }
                        
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}