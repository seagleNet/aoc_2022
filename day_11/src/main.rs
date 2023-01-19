use file_utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Monkey {
    items: Vec<f32>,
    operation: (char, String),
    test_divisible: f32,
    true_dest: i32,
    false_dest: i32,
    activity_count: i32,
}

fn parse_input(lines: &Vec<String>) -> Monkey {
    let mut input_items = Vec::new();
    let mut input_operation = ('x', "num".to_string());
    let mut input_test_divisible = 0.0;
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
                        .map(|f| f.parse::<f32>().unwrap())
                        .collect();
                }
                "Operation" => {
                    let input: Vec<&str> = captures["value"].split_whitespace().collect();
                    input_operation = (input[3].parse::<char>().unwrap(), input[4].to_string());
                }
                "Test" => {
                    let input: Vec<&str> = captures["value"].split_whitespace().collect();
                    input_test_divisible = input[2].parse::<f32>().unwrap();
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

fn inspect_item(item: &f32, operation: &(char, String)) -> f32 {
    let op = operation.0;
    let by_in = operation.1.clone();
    let by = if by_in == "old".to_string() {
        *item
    } else if by_in.parse::<f32>().is_ok() {
        by_in.parse::<f32>().unwrap()
    } else {
        panic!("cannot do maths with {:?}", by_in)
    };

    let worry = match op {
        '*' => item * by,
        '+' => item + by,
        _ => panic!("no operation for {:?}", op),
    };

    (worry / 3.0).floor()
}

fn divisible(worry_level: &f32, test: &f32) -> bool {
    if worry_level % test == 0.0 {
        true
    } else {
        false
    }
}

fn monkey_turn(monkey_stats: &Monkey) -> Vec<(i32, f32)> {
    let mut actions: Vec<(i32, f32)> = Vec::new();

    for item in &monkey_stats.items {
        let worry_level = inspect_item(&item, &monkey_stats.operation);
        let destination = if divisible(&worry_level, &monkey_stats.test_divisible) {
            monkey_stats.true_dest
        } else {
            monkey_stats.false_dest
        };

        actions.push((destination, worry_level));
    }

    actions
}

fn pt1(lines: Vec<String>, rounds: i32) -> i32 {
    let mut monkey_id = 0;
    let mut monkey_file: Vec<String> = Vec::new();
    let mut monkey_list: BTreeMap<i32, Monkey> = BTreeMap::new();

    for line in lines {
        monkey_file.push(line.clone());
        if line.is_empty() {
            monkey_list.insert(monkey_id, parse_input(&monkey_file));
            monkey_file = Vec::new();
            monkey_id += 1;
        }
    }
    monkey_list.insert(monkey_id, parse_input(&monkey_file));

    for _ in 0..rounds {
        for monkey in monkey_list.keys().cloned().collect::<Vec<i32>>() {
            let actions = monkey_turn(monkey_list.get(&monkey).unwrap());

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
    }

    let activity_counts: BTreeSet<i32> = monkey_list
        .values()
        .into_iter()
        .map(|f| f.activity_count)
        .collect();

    activity_counts
        .into_iter()
        .rev()
        .take(2)
        .into_iter()
        .fold(1, |acc, x| acc * x)
}

fn main() {
    let lines = lines_from_file("./day_11/input.txt");

    println!("result pt1: {}", pt1(lines.clone(), 20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines = lines_from_file("./example.txt");

        let result_pt1 = pt1(lines.clone(), 20);
        println!("{:?}", result_pt1);
        assert_eq!(10605, result_pt1);
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
                monkies.insert(monkey_id, parse_input(&monkey_file, &monkey_id));
                monkey_file = Vec::new();
                monkey_id += 1;
            }
        }

        println!("{:?}", monkies);
    }

    #[test]
    fn test_monkey_turn() {
        let monkey = Monkey {
            items: vec![79.0, 98.0],
            operation: ('*', "19".to_string()),
            test_divisible: 23.0,
            true_dest: 2,
            false_dest: 3,
            activity_count: 0,
        };

        assert_eq!(vec![(3, 500.0), (3, 620.0)], monkey_turn(&monkey));
    }
}
