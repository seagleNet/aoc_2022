use file_utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: (char, String),
    test_divisible: i32,
    true_dest: i32,
    false_dest: i32,
    activity_count: i32,
}

fn parse_input(lines: &Vec<String>) -> Monkey {
    let mut input_items = Vec::new();
    let mut input_operation = ('x', "num".to_string());
    let mut input_test_divisible = 0;
    let mut input_true_dest = 0;
    let mut input_false_dest = 0;

    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?:  )*(?P<key>[a-zA-Z0-1 ]+): ?(?P<value>.*)$").unwrap();
    }

    for line in lines {
        if let Some(captures) = RE.captures(line.as_str()) {
            match &captures["key"] {
                "Starting items" => {
                    input_items = captures["value"]
                        .split(", ")
                        .map(|f| f.parse::<i64>().unwrap())
                        .collect();
                }
                "Operation" => {
                    let input: Vec<&str> = captures["value"].split_whitespace().collect();
                    input_operation = (input[3].parse::<char>().unwrap(), input[4].to_string());
                }
                "Test" => {
                    let input: Vec<&str> = captures["value"].split_whitespace().collect();
                    input_test_divisible = input[2].parse::<i32>().unwrap();
                }
                "If true" => {
                    let input: Vec<&str> = captures["value"].split_whitespace().collect();
                    input_true_dest = input[3].parse::<i32>().unwrap();
                }
                "If false" => {
                    let input: Vec<&str> = captures["value"].split_whitespace().collect();
                    input_false_dest = input[3].parse::<i32>().unwrap();
                }
                _ => (),
            }
        }
    }

    Monkey {
        items: input_items,
        operation: input_operation,
        test_divisible: input_test_divisible,
        true_dest: input_true_dest,
        false_dest: input_false_dest,
        activity_count: 0,
    }
}

fn inspect_item(item: &i64, operation: &(char, String), relief: &bool, modulo: &i64) -> i64 {
    let op = operation.0;
    let by_in = operation.1.clone();
    let by = if by_in == "old".to_string() {
        *item
    } else if by_in.parse::<i64>().is_ok() {
        by_in.parse::<i64>().unwrap()
    } else {
        panic!("cannot do maths with {:?}", by_in)
    };

    let worry = match op {
        '*' => item * by,
        '+' => item + by,
        _ => panic!("no operation for {:?}", op),
    };

    if relief == &true {
        worry.div_euclid(3)
    } else {
        worry % modulo
    }
}

fn divisible(worry_level: &i64, test: &i32) -> bool {
    if worry_level % *test as i64 == 0 {
        true
    } else {
        false
    }
}

fn monkey_turn(monkey_stats: &Monkey, relief: &bool, modulo: &i64) -> Vec<(i32, i64)> {
    let mut actions: Vec<(i32, i64)> = Vec::new();

    for item in &monkey_stats.items {
        let worry_level = inspect_item(&item, &monkey_stats.operation, &relief, &modulo);
        let destination = if divisible(&worry_level, &monkey_stats.test_divisible) {
            monkey_stats.true_dest
        } else {
            monkey_stats.false_dest
        };

        actions.push((destination, worry_level));
    }

    actions
}

fn pt1_pt2(lines: Vec<String>, rounds: i32, relief: bool) -> i64 {
    let mut monkey_id = 0;
    let mut monkey_file: Vec<String> = Vec::new();
    let mut monkey_list: BTreeMap<i32, Monkey> = BTreeMap::new();

    for line in lines {
        if line.is_empty() {
            monkey_list.insert(monkey_id, parse_input(&monkey_file));
            monkey_file = Vec::new();
            monkey_id += 1;
        }
        monkey_file.push(line.clone());
    }
    monkey_list.insert(monkey_id, parse_input(&monkey_file));

    let modulo: i64 = monkey_list
        .values()
        .into_iter()
        .map(|f| f.test_divisible as i64)
        .fold(1, |acc, x| acc * x);

    // println!("Modulo: {:?}", modulo);

    for _round in 0..rounds {
        for monkey in monkey_list.keys().cloned().collect::<Vec<i32>>() {
            let actions = monkey_turn(monkey_list.get(&monkey).unwrap(), &relief, &modulo);

            monkey_list
                .entry(monkey)
                .and_modify(|v| v.activity_count += actions.len() as i32);

            for action in actions {
                monkey_list
                    .entry(action.0)
                    .and_modify(|v| v.items.push(action.1));
            }

            monkey_list
                .entry(monkey)
                .and_modify(|v| v.items = Vec::new());
        }
        // println!("# Round: {:?}", _round + 1);
        // println!("{:?}", monkey_list);
    }

    for monkey in &monkey_list {
        println!("Monkey {:?}: {:?}", monkey.0, monkey.1.activity_count);
    }

    let activity_counts: BTreeSet<i64> = monkey_list
        .values()
        .into_iter()
        .map(|f| f.activity_count as i64)
        .collect();

    activity_counts
        .into_iter()
        .rev()
        .take(2)
        .fold(1, |acc, x| acc * x)
}

fn main() {
    let lines = lines_from_file("./day_11/input.txt");

    println!("result pt1: {}", pt1_pt2(lines.clone(), 20, true));
    println!("result pt2: {}", pt1_pt2(lines.clone(), 10000, false));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines = lines_from_file("./example.txt");

        let result_pt1 = pt1_pt2(lines.clone(), 20, true);
        assert_eq!(10605, result_pt1);
        let result_pt2_1 = pt1_pt2(lines.clone(), 1, false);
        assert_eq!(24, result_pt2_1);
        let result_pt2_2 = pt1_pt2(lines.clone(), 20, false);
        assert_eq!(10197, result_pt2_2);
        let result_pt2_3 = pt1_pt2(lines.clone(), 10000, false);
        assert_eq!(2713310158, result_pt2_3);
    }

    #[test]
    fn test_parse_input() {
        let lines = lines_from_file("./example.txt");
        let mut monkey_id = 0;
        let mut monkey_file: Vec<String> = Vec::new();
        let mut monkies: BTreeMap<i32, Monkey> = BTreeMap::new();

        for line in lines {
            monkey_file.push(line.clone());
            if line.is_empty() {
                monkies.insert(monkey_id, parse_input(&monkey_file));
                monkey_file = Vec::new();
                monkey_id += 1;
            }
        }

        println!("{:?}", monkies);
    }

    #[test]
    fn test_monkey_turn() {
        let monkey = Monkey {
            items: vec![79, 98],
            operation: ('*', "19".to_string()),
            test_divisible: 23,
            true_dest: 2,
            false_dest: 3,
            activity_count: 0,
        };

        assert_eq!(vec![(3, 500), (3, 620)], monkey_turn(&monkey, &true, &1));
    }
}
