use file_utils::lines_from_file;
use std::collections::{HashMap, VecDeque};

fn parse_cols(lines: &Vec<String>) -> Vec<String> {
    let mut cols: Vec<String> = Vec::new();
    for line in lines {
        let mut col_index: usize = 0;
        for l in line.chars() {
            if cols.len() <= col_index {
                cols.push(String::new());
            }
            cols[col_index].push(l);
            col_index += 1;
        }
    }
    cols
}

fn to_digits(line: String) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for ch in line.chars() {
        if let Some(digit) = ch.to_digit(10) {
            result.push(digit.try_into().unwrap());
        }
    }
    result
}

fn parse_trees_in_line(line: String) -> HashMap<i32, i32> {
    let trees: VecDeque<i32> = VecDeque::from(to_digits(line));
    let highest_overall = trees.iter().max().unwrap();
    let last_pos: i32 = trees.len().try_into().unwrap();
    let first_tree = trees.clone().pop_front().unwrap();
    let last_tree = trees.clone().pop_back().unwrap();
    let mut result: HashMap<i32, i32> = HashMap::new();

    result.insert(0, first_tree);
    result.insert(last_pos - 1, last_tree);

    let mut pos = 0;
    let mut highest = first_tree;
    let mut trees_front = trees.clone();
    while trees_front.len() != 0 {
        let tree = trees_front.pop_front().unwrap();
        if tree > highest {
            result.insert(pos, tree);
            highest = tree;
            if tree == *highest_overall {
                break;
            }
        }
        pos += 1;
    }

    let mut pos = last_pos - 1;
    let mut highest = last_tree;
    let mut trees_back = trees.clone();
    while trees_back.len() != 0 {
        let tree = trees_back.pop_back().unwrap();
        if tree > highest {
            result.insert(pos, tree);
            highest = tree;
            if tree == *highest_overall {
                break;
            }
        }
        pos -= 1;
    }

    result
}

fn pt1(lines: Vec<String>, cols: Vec<String>) -> i32 {
    let mut tree_index: HashMap<(i32, i32), i32> = HashMap::new();

    let mut line_index: i32 = 0;
    for line in lines {
        let parsed_trees = parse_trees_in_line(line);
        for tree in parsed_trees {
            tree_index.insert((tree.0, line_index), tree.1);
        }
        line_index += 1;
    }

    let mut col_index: i32 = 0;
    for col in cols {
        let parsed_trees = parse_trees_in_line(col);
        for tree in parsed_trees {
            tree_index.insert((col_index, tree.0), tree.1);
        }
        col_index += 1;
    }

    println!("{:?}", tree_index);
    tree_index.keys().len().try_into().unwrap()
}

fn main() {
    let lines = lines_from_file("./day_8/day_8.in");
    let cols = parse_cols(&lines);

    println!("result pt1: {}", pt1(lines, cols));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines: Vec<String> = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];
        let cols = parse_cols(&lines);

        let result = pt1(lines, cols);
        println!("{:?}", result);
        assert_eq!(21, result);
    }

    #[test]
    fn test_parse_rows() {
        let lines: Vec<String> = vec![
            "30373".to_string(),
            "25512".to_string(),
            "65332".to_string(),
            "33549".to_string(),
            "35390".to_string(),
        ];

        let expected: Vec<String> = vec![
            "32633".to_string(),
            "05535".to_string(),
            "35353".to_string(),
            "71349".to_string(),
            "32290".to_string(),
        ];

        println!("{:?}", parse_cols(&lines));
        assert_eq!(expected, parse_cols(&lines));
    }

    #[test]
    fn test_parse_trees_in_line() {
        println!("{:?}", parse_trees_in_line("30373".to_string()));
        println!("{:?}", parse_trees_in_line("71349".to_string()));
    }
}
