use file_utils::lines_from_file;
use serde_json::Value;
use std::collections::VecDeque;

fn parse_nested_vector(input: &Value) -> Vec<Value> {
    match input {
        Value::Array(arr) => arr.to_vec(),
        _ => vec![input.clone()],
    }
}

fn flatten_nested_vector(input: &Value) -> Vec<Value> {
    let nested_vector = parse_nested_vector(input);
    let mut result = Vec::new();
    for item in nested_vector {
        match item {
            Value::Array(arr) => {
                let flattened_arr = flatten_nested_vector(&Value::Array(arr));
                result.push(Value::Array(flattened_arr));
            }
            _ => result.push(item),
        }
    }
    result
}

fn parse_lines(lines: &Vec<String>) -> Vec<(VecDeque<Value>, VecDeque<Value>)> {
    let mut pair: Vec<Vec<Value>> = Vec::new();
    let mut pairs: Vec<(VecDeque<Value>, VecDeque<Value>)> = Vec::new();

    for line in lines {
        let value: Value = serde_json::from_str(&line).unwrap_or_default();
        if !value.is_null() {
            pair.push(flatten_nested_vector(&value));
        }
        if pair.len() == 2 {
            pairs.push((
                VecDeque::from(pair[0].clone()),
                VecDeque::from(pair[1].clone()),
            ));
            pair.clear();
        }
    }
    pairs
}

fn main() {
    let lines: Vec<String> = lines_from_file("./day_13/input.txt");
    let pairs = parse_lines(&lines);
    for pair in pairs.iter().enumerate() {
        println!("pair {} l: {:?}",pair.0, pair.1.0);
        println!("pair {} r: {:?}", pair.0, pair.1.1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines: Vec<String> = lines_from_file("./example.txt");
    }

    #[test]
    fn test_parse() {
        let input: String = "[1,1,3,1,1]".to_string();
        println!("{:?}", test_parse_line(input.clone()));
        assert_eq!(vec![1, 1, 3, 1, 1], test_parse_line(input));

        let input: String = "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string();
        println!("{:?}", test_parse_line(input));
        let input: String = "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();
        println!("{:?}", test_parse_line(input));
    }

    fn test_parse_line(input: String) -> Vec<Value> {
        let value: Value = serde_json::from_str(&input).unwrap();
        flatten_nested_vector(&value)
    }
}
