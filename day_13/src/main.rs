use file_utils::lines_from_file;
use serde_json::Value;

fn parse_nested_vector(v: &Value) -> Vec<Value> {
    match v {
        Value::Array(arr) => arr.to_vec(),
        _ => vec![v.clone()],
    }
}

fn flatten_nested_vector(v: &Value) -> Vec<Value> {
    let nested_vector = parse_nested_vector(v);
    let mut result = vec![];
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

fn main() {
    let grid: Vec<String> = lines_from_file("./day_12/input.txt");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let grid: Vec<String> = lines_from_file("./example.txt");
    }

    #[test]
    fn test_parse() {
        let input: String = "[1,1,3,1,1]".to_string();
        println!("{:?}", test_parse_line(input.clone()));
        assert_eq!(vec![1,1,3,1,1], test_parse_line(input));

        let input: String = "[1,[2,[3,[4,[5,6,7]]]],8,9]".to_string();
        println!("{:?}", test_parse_line(input));
        let input: String = "[1,[2,[3,[4,[5,6,0]]]],8,9]".to_string();
        println!("{:?}", test_parse_line(input));
    }

    fn test_parse_line(input: String) -> Vec<Value> {
        let v: Value = serde_json::from_str(&input).unwrap();
        flatten_nested_vector(&v)
    }
}
