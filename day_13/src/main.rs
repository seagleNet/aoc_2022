use file_utils::lines_from_file;
use serde_json::{json, Value};

fn parse_json(lines: &Vec<String>) -> Vec<(Value, Value)> {
    let mut pair: Vec<Value> = Vec::new();
    let mut pairs: Vec<(Value, Value)> = Vec::new();

    for line in lines {
        let value: Value = serde_json::from_str(&line).unwrap_or_default();
        if !value.is_null() {
            pair.push(value);
        }
        if pair.len() == 2 {
            pairs.push((pair[0].clone(), pair[1].clone()));
            pair.clear();
        }
    }
    return pairs;
}

fn right_order(left: Value, right: Value) -> i32 {
    for (value_left, value_right) in left
        .as_array()
        .unwrap_or(&vec![Value::Null])
        .iter()
        .zip(right.as_array().unwrap_or(&vec![Value::Null]).iter())
    {
        println!(
            "comparing values\nl: {:?}\nr: {:?}",
            value_left, value_right
        );
        match (value_left, value_right) {
            (_, Value::Null) => return 0,
            (Value::Null, _) => return 1,
            (Value::Array(_), Value::Array(_)) => {
                return right_order(value_left.clone(), value_right.clone())
            }
            (Value::Number(_), Value::Array(_)) => {
                return right_order(json!(value_left), value_right.clone())
            }
            (Value::Array(_), Value::Number(_)) => {
                return right_order(value_left.clone(), json!([value_right]))
            }
            (Value::Number(vl), Value::Number(vr)) => {
                if vl.as_i64().unwrap() > vr.as_i64().unwrap() {
                    println!("return false - l:{:?} r:{:?}", vl, vr);
                    return 0;
                } else if vl.as_i64().unwrap() < vr.as_i64().unwrap() {
                    return 1;
                } else if vl.as_i64().unwrap() == vr.as_i64().unwrap() {
                } else {
                    panic!("cannot compare values l:{:?} r:{:?}", vl, vr);
                }
            }
            _ => panic!(
                "cannot compare values l:{:?} r:{:?}",
                value_left, value_right
            ),
        }
    }
    panic!("cannot compare pairs\nl:{:?}\nr:{:?}", left, right)
}

fn part_1(pairs: Vec<(Value, Value)>) -> i32 {
    let mut result = 0;
    let mut pair_no = 0;
    for pair in pairs {
        println!(
            "> start pair no: {:?}\n> input l: {:?}\n> input r: {:?}",
            pair_no, pair.0, pair.1
        );
        result += right_order(pair.0, pair.1);
        pair_no += 1;
    }
    return result;
}

fn main() {
    let lines: Vec<String> = lines_from_file("./day_13/input.txt");
    let pairs = parse_json(&lines);

    println!("{}", part_1(pairs));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines: Vec<String> = lines_from_file("./example.txt");
        let pairs = parse_json(&lines);

        println!("{}", part_1(pairs));
    }

    #[test]
    fn test_parse_json() {
        let mut lines: Vec<String> = Vec::new();
        lines.push("[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string());
        lines.push("[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string());
        lines.push("[[[]]]".to_string());
        lines.push("[[]]".to_string());
        let pairs = parse_json(&lines);
        for pair in pairs {
            println!("l: {:?}\nr: {:?}", pair.0, pair.1);
        }
    }
}
