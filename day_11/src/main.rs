use file_utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

struct Monkey {
    id: i32,
    items: Vec<i32>,
    operation: (String, String),
    test_divisible: i32,
    true_dest: i32,
    false_dest: i32,
    activity_count: i32,
}

fn parse_input(lines: &Vec<String>, monkey_id: &i32) -> Monkey {
    let mut input_items = Vec::new();
    let mut input_operation = ('x'.to_string(), "num".to_string());
    let mut input_test_divisible = 0;
    let mut input_true_dest = 0;
    let mut input_false_dest = 0;
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(?:  )*(?<key>[a-zA-Z0-1 ]+): ?(?<value>.*)$").unwrap();
    }

    for line in lines {
        if let Some(captures) = RE.captures(line.as_str()) {
            match &captures["key"] {
                "Starting items" => {
                    input_items = captures["value"]
                        .split(", ")
                        .map(|f| f.parse::<i32>().unwrap())
                        .collect();
                }
                "Operation" => {
                    let input: Vec<&str> = captures["value"]
                        .split_whitespace().collect();
                    input_operation = (input[3].to_string(), input[4].to_string())
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
        id: *monkey_id,
        items: input_items,
        operation: input_operation,
        test_divisible: input_test_divisible,
        true_dest: input_true_dest,
        false_dest: input_false_dest,
        activity_count: 0,
    }
}

fn pt1(lines: Vec<String>) -> i32 {
    let mut monkey_id = 0;
    let mut monkey_file: Vec<String> = Vec::new();
    let mut monkies: HashMap<i32, Monkey> = HashMap::new();

    for line in lines {
        monkey_file.push(line.clone());
        if line.is_empty() {
            monkies.insert(monkey_id, parse_input(&monkey_file, &monkey_id));
            monkey_file = Vec::new();
            monkey_id += 1;
        }
    }
    return 10605;
}

fn main() {
    let lines = lines_from_file("./day_11/input.txt");

    println!("result pt1: {}", pt1(lines.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines = lines_from_file("./example.txt");

        let result_pt1 = pt1(lines.clone());
        println!("{:?}", result_pt1);
        assert_eq!(10605, result_pt1);
    }
}
