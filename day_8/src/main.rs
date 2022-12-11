use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Index;
use std::path::Path;
use std::vec;
use rounded_div;
use transpose;

#[warn(unused_variables)]
fn main() {

    let mut forrest_rows: Vec<Vec<u32>> = vec![];
    let mut number_of_visible_trees: u32 = 0;

    if let Ok(lines) = read_lines("./trees.txt") {
        for line in lines {
            if let Ok(value) = line {
                forrest_rows.push(value.chars().map(|t| t.to_digit(10 as u32).unwrap()).collect());
            }

        }
    }

    let rows_number = forrest_rows.len();
    let columns_number = forrest_rows[0].len();

    let mut forrest_columns: Vec<Vec<u32>> = vec![vec![]; columns_number];

    for row in &forrest_rows {
        for (index, column) in row.iter().enumerate() {
            forrest_columns[index].push(column.clone());
        }
    }

    // Part 1

    // for row_id in 0..rows_number {
    //     for col_id in 0..columns_number {
    //         let tree = forrest_rows[row_id][col_id];
    //         if (row_id == 0) | (row_id == rows_number - 1) | (col_id == 0) | (col_id == columns_number - 1) |
    //         (forrest_rows[row_id][..col_id].iter().max().unwrap_or(&0) < &tree) | 
    //         (&tree > forrest_rows[row_id][col_id+1..].iter().max().unwrap_or(&0)) |  
    //         (forrest_columns[col_id][..row_id].iter().max().unwrap_or(&0) < &tree) | 
    //         (&tree > forrest_columns[col_id][row_id+1..].iter().max().unwrap_or(&0))
    //             {
    //                 number_of_visible_trees += 1;
    //             }
    //     }
    // }
    // dbg!(number_of_visible_trees);

    //Part 2

    let mut max_views: Vec<u32> = vec![];
    for row_id in 0..rows_number {
        for col_id in 0..columns_number {
            let tree = forrest_rows[row_id][col_id];

            let mut view_up_score: u32 = 0;
            let mut view_down_score: u32 = 0;
            let mut view_left_score: u32 = 0;
            let mut view_right_score: u32 = 0;
            
            // Calculate up score
            if row_id == 0 {
                view_up_score = 1;
            } else {
                let next_equal_biger_tree = forrest_columns[col_id][..row_id].iter().rev().enumerate().position(|t| t.1>=&tree);
                view_up_score = match next_equal_biger_tree {
                    Some(index) => (index + 1) as u32,
                    None => row_id as u32
                };
            }
            
            // Calculate down score
            if row_id == rows_number - 1 {
                view_down_score = 1;
            } else {
                let next_equal_biger_tree = forrest_columns[col_id][row_id+1..].iter().enumerate().position(|t| t.1>=&tree);
                let max_index = columns_number - 1;
                view_down_score = match next_equal_biger_tree {
                    Some(index) => (index + 1) as u32,
                    None => (max_index - row_id) as u32 
                };
            }
            // Calulate left view
            if col_id == 0 {
                view_left_score = 1;
            } else {
                let next_equal_biger_tree = forrest_rows[row_id][..col_id].iter().rev().enumerate().position(|t| t.1>=&tree);
                view_left_score = match next_equal_biger_tree {
                    Some(index) => (index + 1) as u32,
                    None => col_id as u32
                };
            }
            // Calculate right view
            if (col_id == columns_number - 1) {
                view_right_score = 1;
            } else {
                let next_equal_biger_tree = forrest_rows[row_id][col_id+1..].iter().enumerate().position(|t| t.1>=&tree);
                let max_index = rows_number - 1;
                view_right_score = match next_equal_biger_tree {
                    Some(index) => (index + 1) as u32,
                    None => (max_index - col_id) as u32
                };
            }
            println!("row {} col {}: left: {}, up: {}, right: {}, down: {}", row_id, col_id, view_left_score, view_up_score, view_right_score, view_down_score);
            max_views.push(view_left_score * view_right_score * view_up_score * view_down_score);
        }
    }
    println!("{:#?}", &max_views);
    println!("{:#?}", &max_views.iter().max());
}



fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
