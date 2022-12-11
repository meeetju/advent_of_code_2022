use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct RopeKnot {
    x_position: i32,
    y_position: i32
}

impl RopeKnot {
    fn new() -> Self{
        Self {
            x_position: 0,
            y_position: 0,
        }
    }

    pub fn is_stretched_from(&self, other: &RopeKnot) -> bool {
        ((self.x_position - other.x_position).abs() > 1) | ((self.y_position - other.y_position).abs() > 1)
    }
}

const LEFT: &str = "L";
const UP: &str = "U";
const RIGHT: &str = "R";
const DOWN: &str = "D";

// Part 1
// const NUM_OF_KNOTS: usize = 2;
// const SOURCE_FILE: &str = "./steps.txt";
// Part 2
const NUM_OF_KNOTS: usize = 10;
const SOURCE_FILE: &str = "./steps_2.txt";

fn main() {

    let mut knots: Vec<RopeKnot> = vec![];

    for _ in 0..NUM_OF_KNOTS {
        knots.push(RopeKnot::new());
    }

    let mut last_knot_positions: HashSet<(i32, i32)> = HashSet::new();
    last_knot_positions.insert((knots.last().unwrap().x_position, knots.last().unwrap().y_position));

    if let Ok(lines) = read_lines(SOURCE_FILE) {
        for line in lines {
            if let Ok(value) = line {
                let new_move: Vec<&str> = value.split(" ").into_iter().collect();
                let direction = new_move[0];
                let distance_steps = new_move[1];

                for _ in 0..(distance_steps.to_string().parse::<u8>().unwrap()) {

                    // Head moves
                    match direction {
                        LEFT => knots[0].x_position -= 1,
                        UP => knots[0].y_position += 1,
                        RIGHT => knots[0].x_position += 1,
                        DOWN => knots[0].y_position -= 1,
                        _ => {}
                        
                    }

                    for index in 1..knots.len() {

                        if knots[index].is_stretched_from(&knots[index-1]) {
                            // Same horizontally
                            if knots[index].y_position == knots[index-1].y_position {
                                if knots[index].x_position < knots[index-1].x_position {
                                    knots[index].x_position += 1;
                                } else {
                                    knots[index].x_position -= 1;
                                }
                            }
                            // Same vertically
                            if knots[index].x_position == knots[index-1].x_position {
                                if knots[index].y_position < knots[index-1].y_position {
                                    knots[index].y_position += 1;
                                } else {
                                    knots[index].y_position -= 1;
                                }
                            }
                            // Previous is Right Top
                            if knots[index].x_position < knots[index-1].x_position && knots[index].y_position < knots[index-1].y_position {
                                knots[index].x_position += 1;
                                knots[index].y_position += 1;
                            }
                            // Previous is Left Top
                            if knots[index].x_position > knots[index-1].x_position && knots[index].y_position < knots[index-1].y_position {
                                knots[index].x_position -= 1;
                                knots[index].y_position += 1;
                            }
                            // Previous is Right Down
                            if knots[index].x_position < knots[index-1].x_position && knots[index].y_position > knots[index-1].y_position {
                                knots[index].x_position += 1;
                                knots[index].y_position -= 1;
                            }
                            // Previous is Left Down
                            if knots[index].x_position > knots[index-1].x_position && knots[index].y_position > knots[index-1].y_position {
                                knots[index].x_position -= 1;
                                knots[index].y_position -= 1;
                            }
                        }
                        last_knot_positions.insert((knots.last().unwrap().x_position, knots.last().unwrap().y_position));
                    }
                }
            }
        }
    }
    dbg!(last_knot_positions.len());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
