use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut priorities_sum: u32 = 0;

    if let Ok(lines) = read_lines("./rucksack.txt") {
        // // part_1 start
        // for line in lines {
        //     if let Ok(value) = line {
        //         dbg!(&value);
        //         let double = find_double(&value[..value.len()/2], &value[value.len()/2..]);
        //         priorities_sum += get_priority(double.chars().next().unwrap())
        //     }
        // }
        // // part_1 end
        
        // part_2 start
        let mut rucksacks_contents: Vec<String> = vec![];
        for line in lines {
            if let Ok(value) = line {
                rucksacks_contents.push(value)
            }
        }

        for (index, _) in rucksacks_contents.iter().enumerate().step_by(3) {
            let content_a = rucksacks_contents[index].clone();
            let content_b = rucksacks_contents[index + 1].clone();
            let content_c = rucksacks_contents[index + 2].clone();
            let tripple = find_tripple(&content_a, &content_b, &content_c);
            priorities_sum += get_priority(tripple.chars().next().unwrap());
        }
        // part_2 end
    }
    dbg!(priorities_sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_double(a: &str, b: &str) -> String {
    for char in a.chars() {
        if b.contains(char) {
            dbg!(char);
            return char.to_string()
        }
    }
    "Something went wrong".to_string()
}

fn find_tripple(a: &str, b: &str, c: &str) -> String {
    for char in a.chars() {
        if b.contains(char) && c.contains(char) {
            dbg!(char);
            return char.to_string()
        }
    }
    "Something went wrong".to_string()
}

fn get_priority(letter: char) -> u32 {
    match letter {
        'a'..='z' => (letter as u32) - 96,
        'A'..='Z' => (letter as u32) - 38,
        _ => panic!("Something went wrong!")
    }
}

#[cfg(test)]
mod tests {

    use crate::{get_priority};

    #[test]
    fn test_priorities() {
        assert_eq!(get_priority('a') == 1, true);
        assert_eq!(get_priority('z') == 26, true);
        assert_eq!(get_priority('A') == 27, true);
        assert_eq!(get_priority('Z') == 52, true);
    }
}