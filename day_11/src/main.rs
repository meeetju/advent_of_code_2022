use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

const NUMBER_OF_ROUNDS_1: u64 = 20;
const WORRY_DIV_FACTOR_1: u64 = 3;
const NUMBER_OF_ROUNDS_2: u64 = 10000;
const WORRY_DIV_FACTOR_2: u64 = 1;

#[derive(Clone, Debug)]
enum Method {
    ADD,
    MULTIPLY,
}

impl Default for Method {
    fn default() -> Self { Method::ADD }
}

#[derive(Default, Clone, Debug)]
struct Monkey {
    items: VecDeque<u64>,
    worry_argument: Option<u64>,
    worry_method: Method,
    test_argument: u64,
    test_true_monkey_id: usize,
    test_false_monkey_id: usize,
    inspected_items: u64
}

impl Monkey {
    pub fn add_item(&mut self, item: u64) {
        self.items.push_back(item);
    }

    pub fn inspect(&mut self, worry_div_factor: u64) -> Option<(u64, usize)> {
        let response = match self.items.pop_front() {
            Some(worry_level) => {
                let new_worry_level = self.modify_worry_level(worry_level, worry_div_factor);
                let next_monkey_id = self.pick_next_monkey(new_worry_level);
                self.inspected_items += 1;
                Some((new_worry_level, next_monkey_id))
            },
            None => None
        };
        response

    }

    fn modify_worry_level(&self, item: u64, worry_div_factor: u64) -> u64 {
        let argument: u64 = match self.worry_argument {
            Some(arg) => arg,
            None => item
        };

        let new_worry_level: u64 = match self.worry_method {
            Method::ADD => {item + argument},
            Method::MULTIPLY => {item * argument}
        };

        new_worry_level / worry_div_factor
    }

    fn pick_next_monkey(&self, item: u64) -> usize {
        if item % self.test_argument == 0 {
            return self.test_true_monkey_id
        } else {
            return self.test_false_monkey_id
        }
    }
}

fn main() {

    let mut monkeys: Vec<Monkey> = vec![];
    let mut monkeys_number: usize = 0;

    if let Ok(lines) = read_lines("./monkey_actions.txt") {
        for line in lines {
            if let Ok(value) = line {
                let read_words = value.trim().split(" ").collect::<Vec<&str>>();
                dbg!(&read_words);
                match read_words[0] {
                    "Monkey" => {        
                        let new_monkey = Monkey::default();
                        monkeys.push(new_monkey);
                        monkeys_number += 1;
                    },
                    "Starting" => {
                        let items: Vec<u64> = read_words[2..].iter().map(|i| i.replace(",","").parse::<u64>().unwrap()).collect();
                        for item in items {
                            monkeys[monkeys_number-1].add_item(item);
                        }
                    },
                    "Operation:" => {
                        if read_words[4].contains("*") {
                            monkeys[monkeys_number-1].worry_method = Method::MULTIPLY;
                        }
                        match read_words[5].parse::<u64>() {
                            Ok(value) => {monkeys[monkeys_number-1].worry_argument = Some(value)},
                            Err(_) => {monkeys[monkeys_number-1].worry_argument = None}
                        }
                    },
                    "Test:" => {
                        monkeys[monkeys_number-1].test_argument = read_words[3].parse::<u64>().unwrap();
                    },
                    "If" => {
                        match read_words[1] {
                            "true:" => monkeys[monkeys_number-1].test_true_monkey_id = read_words[5].parse::<usize>().unwrap(),
                            "false:" => monkeys[monkeys_number-1].test_false_monkey_id = read_words[5].parse::<usize>().unwrap(),
                            _ => panic!("Something went wrong!")
                        }
                    },
                    _ => {}
                }
            } 
        }
    }

    println!("**** Part 1 *****");

    let mut monkeys_1 = monkeys.clone();

    for _ in 0..NUMBER_OF_ROUNDS_1 {
        for monkey_index in 0..monkeys_number {
            loop {
                match monkeys_1[monkey_index].inspect(WORRY_DIV_FACTOR_1) {
                    Some((item, monkey_id)) => {
                        monkeys_1[monkey_id].add_item(item);
                    },
                    None => {break;}
                }
            }
        }
    }

    let mut inspected_items: Vec<u64> = vec![];
    for monkey in monkeys_1 {
        inspected_items.push(monkey.clone().inspected_items);
    }

    inspected_items.sort();
    println!("Monkey business: {} ", inspected_items.pop().unwrap() * inspected_items.pop().unwrap());

    println!("**** Part 2 *****");
    
    let mut monkeys_2 = monkeys.clone();

    let mod_val: u64 = monkeys.iter().map(|monkey| monkey.test_argument).collect::<Vec<u64>>().iter().product();

    for _ in 0..NUMBER_OF_ROUNDS_2 {
        for monkey_index in 0..monkeys_number {
            loop {
                match monkeys_2[monkey_index].inspect(WORRY_DIV_FACTOR_2) {
                    Some((item, monkey_id)) => {
                        monkeys_2[monkey_id].add_item(item % mod_val); // I didn't know about this trick
                    },
                    None => {break;}
                }
            }
        }
    }

    let mut inspected_items: Vec<u64> = vec![];
    for monkey in monkeys_2 {
        inspected_items.push(monkey.clone().inspected_items);
    }

    inspected_items.sort();
    println!("Monkey business: {} ", inspected_items.pop().unwrap() * inspected_items.pop().unwrap());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())

}
