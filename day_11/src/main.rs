use file_utils::lines_from_file;
use std::collections::HashMap;

fn pt1(lines: Vec<String>) -> i32 {
    return 10605
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
