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

fn right_order(left: &mut VecDeque<Value>, right: &mut VecDeque<Value>) -> bool {
    let value_left;
    let value_right;
    println!("input l:{:?} r:{:?}", left, right);

    if left.is_empty() && right.is_empty() {
        println!("return true - l:{:?} r:{:?}", left, right);
        return true;
    } else if left.is_empty() {
        println!("return true - l:{:?} r:{:?}", left, right);
        return true;
    } else if right.is_empty() {
        println!("return false - l:{:?} r:{:?}", left, right);
        return false;
    } else {
        value_left = left.pop_front().unwrap();
        value_right = right.pop_front().unwrap();
    }
    println!("values l:{:?} r:{:?}", value_left, value_right);

    if value_left.is_array() && value_right.is_number() {
        left.push_front(value_left);
        right.push_front(Value::Array(vec![value_right]));
        return right_order(left, right);
    } else if value_left.is_number() && value_right.is_array() {
        left.push_front(Value::Array(vec![value_left]));
        right.push_front(value_right);
        return right_order(left, right);
    } else if value_left.is_array() && value_right.is_array() {
        for (vl, vr) in value_left
            .as_array()
            .unwrap()
            .iter()
            .zip(value_right.as_array().unwrap().iter())
        {
            match (vl, vr) {
                (Value::Array(vl), Value::Array(vr)) => {}
                (Value::Number(_), Value::Null) => return false,
                (Value::Number(vl), Value::Number(vr)) => {
                    if vl.as_i64().unwrap() > vr.as_i64().unwrap() {
                        println!("return false - l:{:?} r:{:?}", vl, vr);
                        return false;
                    }
                }
                _ => panic!("cannot compare values l:{:?} r:{:?}", vl, vr),
            }
        }
    } else if value_left.is_number() && value_right.is_number() {
        if value_left.as_i64().unwrap() > value_right.as_i64().unwrap() {
            println!("return false - l:{:?} r:{:?}", value_left, value_right);
            return false;
        }
    } else {
        panic!(
            "cannot figure out order for values l:{:?} r:{:?}",
            value_left, value_right
        )
    }
    return right_order(left, right);
}

fn part_1(pairs: Vec<(VecDeque<Value>, VecDeque<Value>)>) -> i32 {
    let mut result = 0;
    let mut pair_no = 1;
    for mut pair in pairs {
            println!("> check pair no {:?}", pair_no);
        if right_order(&mut pair.0, &mut pair.1) {
            println!("< resutl pair no {:?} OK", pair_no);
            result += pair_no;
        } else {
            println!("< resutl pair no {:?} NOK", pair_no);
        }
        pair_no += 1;
    }
    result
}
fn main() {
    let lines: Vec<String> = lines_from_file("./day_13/input.txt");
    let pairs = parse_lines(&lines);

    println!("{}", part_1(pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines: Vec<String> = lines_from_file("./example.txt");
        let pairs = parse_lines(&lines);

        println!("{}", part_1(pairs));
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
        let input: String = "[[[]]]".to_string();
        println!("{:?}", test_parse_line(input));
        let input: String = "[[]]".to_string();
        println!("{:?}", test_parse_line(input));
    }

    fn test_parse_line(input: String) -> Vec<Value> {
        let value: Value = serde_json::from_str(&input).unwrap();
        flatten_nested_vector(&value)
    }
}
