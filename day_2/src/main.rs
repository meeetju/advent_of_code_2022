use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {

    let mut player_1_pick: Items;
    let mut player_2_pick: Items;
    let mut player_2_score: u32 = 0;

    if let Ok(lines) = read_lines("./puzzle.txt") {
        for line in lines {
            if let Ok(value) = line {
                // (player_1_pick, player_2_pick) = get_picks(&value);
                (player_1_pick, player_2_pick) = get_cheated_picks(&value);
                update_player_2_score(player_1_pick, player_2_pick, &mut player_2_score);
            }
        }
        dbg!(player_2_score);
    }
}

fn update_player_2_score(player_1_pick: Items, player_2_pick: Items, player_2_score: &mut u32) {
    if player_1_pick == player_2_pick {
        *player_2_score += 3;
    } else if player_1_pick < player_2_pick{
        *player_2_score += 6;
    }
    match player_2_pick {
        Items::Rock => *player_2_score += 1,
        Items::Paper => *player_2_score += 2,
        Items::Scissors => *player_2_score += 3
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_picks(game: &str) -> (Items, Items) {
    let player_1_items: HashMap<String, Items> = HashMap::from([("A".to_string(), Items::Rock), ("B".to_string(), Items::Paper), ("C".to_string(), Items::Scissors)]);
    let player_2_items: HashMap<String, Items> = HashMap::from([("X".to_string(), Items::Rock), ("Y".to_string(), Items::Paper), ("Z".to_string(), Items::Scissors)]);
    (player_1_items.get(&game[..1]).unwrap().clone(), player_2_items.get(&game[2..3]).unwrap().clone())
}

fn get_cheated_picks(game: &str) -> (Items, Items) {
    //X loose, Y draw, Z win
    let player_1_items: HashMap<String, Items> = HashMap::from([("A".to_string(), Items::Rock), ("B".to_string(), Items::Paper), ("C".to_string(), Items::Scissors)]);
    let player_1_item = player_1_items.get(&game[..1]).unwrap().clone();
    let player_2_item = match &game[2..3]{
        "Y" => player_1_item.clone(),
        "X" => match player_1_item {
            Items::Rock => Items::Scissors,
            Items::Paper => Items::Rock,
            Items::Scissors => Items::Paper
        },
        "Z" => match player_1_item {
            Items::Rock => Items::Paper,
            Items::Paper => Items::Scissors,
            Items::Scissors => Items::Rock
        },
        &_ => panic!("Something went really wrong!")
    };
    (player_1_item, player_2_item)
}

#[derive(Eq, Copy, Clone, Debug, Ord)]
enum Items {
    Paper,
    Rock,
    Scissors,
}

impl PartialEq for Items {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Paper, Self::Paper) | (Self::Rock, Self::Rock) | (Self::Scissors, Self::Scissors) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Items {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
    fn lt(&self, other: &Self) -> bool { 
        match (self, other) {
            (Self::Paper, Self::Scissors) | (Self::Scissors, Self::Rock) | (Self::Rock, Self::Paper) => true,
            _ => false,
        }
    }
    fn gt(&self, other: &Self) -> bool { 
        match (self, other) {
            (Self::Paper, Self::Rock) | (Self::Scissors, Self::Paper) | (Self::Rock, Self::Scissors) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{Items, get_picks, get_cheated_picks};

    #[test]
    fn comparisons() {
    assert_eq!(Items::Paper == Items::Paper, true);
    assert_eq!(Items::Rock == Items::Rock, true);
    assert_eq!(Items::Scissors == Items::Scissors, true);
    assert_eq!(Items::Paper > Items::Rock, true);
    assert_eq!(Items::Rock > Items::Scissors, true);
    assert_eq!(Items::Scissors > Items::Paper, true);
    assert_eq!(Items::Scissors < Items::Rock, true);
    assert_eq!(Items::Rock < Items::Paper, true);
    assert_eq!(Items::Paper < Items::Scissors, true);
    assert_eq!(Items::Scissors == Items::Paper, false);
    }

    #[test]
    fn picks() {
        // Normal game
        assert_eq!((Items::Paper, Items::Paper) == get_picks("B Y"), true);
        assert_eq!((Items::Scissors, Items::Rock) == get_picks("C X"), true);
        assert_eq!((Items::Rock, Items::Scissors) == get_picks("A Z"), true);
        assert_eq!((Items::Rock, Items::Rock) == get_picks("A Z"), false);
        // Cheated game: X loose, Y draw, Z win
        assert_eq!((Items::Paper, Items::Rock) == get_cheated_picks("B X"), true);
        assert_eq!((Items::Paper, Items::Paper) == get_cheated_picks("B Y"), true);
        assert_eq!((Items::Paper, Items::Scissors) == get_cheated_picks("B Z"), true);
    }
}