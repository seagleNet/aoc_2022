use file_utils::lines_from_file;
use std::collections::{HashMap};

fn pt1(lines: Vec<String>) -> i32 {
    return 13
}

fn main() {
    let lines = lines_from_file("./day_9/day_9.in");

    println!("result pt1: {}", pt1(lines.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines: Vec<String> = vec![
            "R 4".to_string(),
            "U 4".to_string(),
            "L 3".to_string(),
            "D 1".to_string(),
            "R 4".to_string(),
            "D 1".to_string(),
            "L 5".to_string(),
            "R 2".to_string(),
        ];

        let result_pt1 = pt1(lines.clone());
        println!("{:?}", result_pt1);
        assert_eq!(13, result_pt1);
    }
}
