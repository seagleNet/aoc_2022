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

fn visible_trees_from_edges(line: String) -> HashMap<i32, i32> {
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

fn view_distances_per_tree(line: String) -> HashMap<i32, Vec<i32>> {
    let trees: Vec<i32> = Vec::from(to_digits(line));
    let last_pos = trees.len() - 1;
    let mut result: HashMap<i32, Vec<i32>> = HashMap::new();

    for pos in 0..=last_pos {
        let tree = trees[pos];

        let mut view_distance_front = 0;
        if pos != 0 {
            for pos_front in (0..pos).rev() {
                if trees[pos_front] < tree {
                    view_distance_front += 1;
                } else {
                    view_distance_front += 1;
                    break;
                }
            }
        }

        let mut view_distance_back = 0;
        if pos != last_pos {
            for pos_back in pos + 1..=last_pos {
                if trees[pos_back] < tree {
                    view_distance_back += 1;
                } else {
                    view_distance_back += 1;
                    break;
                }
            }
        }

        let view_distances = vec![view_distance_front, view_distance_back];

        result.insert(pos.try_into().unwrap(), view_distances);
    }

    result
}

fn calc_scenic_score(input: Vec<i32>) -> i32 {
    let mut score = 1;
    for distance in input {
        if distance > 0 {
            score *= distance;
        }
    }

    score
}

fn pt1(lines: Vec<String>, cols: Vec<String>) -> i32 {
    let mut tree_index: HashMap<(i32, i32), i32> = HashMap::new();

    let mut line_index: i32 = 0;
    for line in lines {
        let visible_trees = visible_trees_from_edges(line);
        for tree in visible_trees {
            tree_index.insert((tree.0, line_index), tree.1);
        }
        line_index += 1;
    }

    let mut col_index: i32 = 0;
    for col in cols {
        let visible_trees = visible_trees_from_edges(col);
        for tree in visible_trees {
            tree_index.insert((col_index, tree.0), tree.1);
        }
        col_index += 1;
    }

    tree_index.keys().len().try_into().unwrap()
}

fn pt2(lines: Vec<String>, cols: Vec<String>) -> i32 {
    let mut tree_index: HashMap<(i32, i32), Vec<i32>> = HashMap::new();
    let mut scenic_scores: HashMap<(i32, i32), i32> = HashMap::new();

    let mut line_index: i32 = 0;
    for line in lines {
        let trees = view_distances_per_tree(line);
        for tree in trees {
            tree_index.insert((tree.0, line_index), tree.1);
        }
        line_index += 1;
    }

    let mut col_index: i32 = 0;
    for col in cols {
        let trees = view_distances_per_tree(col);
        for tree in trees {
            for distance in tree.1 {
                tree_index
                    .entry((col_index, tree.0))
                    .or_insert(vec![distance])
                    .push(distance);
            }
        }
        col_index += 1;
    }

    for tree in tree_index {
        scenic_scores.insert(tree.0, calc_scenic_score(tree.1));
    }

    *scenic_scores.values().max().unwrap()
}

fn main() {
    let lines = lines_from_file("./day_8/day_8.in");
    let cols = parse_cols(&lines);

    println!("result pt1: {}", pt1(lines.clone(), cols.clone()));
    println!("result pt2: {}", pt2(lines, cols));
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

        let result_pt1 = pt1(lines.clone(), cols.clone());
        println!("{:?}", result_pt1);
        assert_eq!(21, result_pt1);

        let result_pt2 = pt2(lines, cols);
        println!("{:?}", result_pt2);
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
    fn test_visible_trees_from_edges() {
        println!("{:?}", visible_trees_from_edges("30373".to_string()));
        println!("{:?}", visible_trees_from_edges("71349".to_string()));
    }

    #[test]
    fn test_view_distances_per_tree() {
        println!("{:?}", view_distances_per_tree("25512".to_string()));
        println!("{:?}", view_distances_per_tree("35353".to_string()));
    }

    #[test]
    fn test_calc_scenic_score() {
        assert_eq!(4, calc_scenic_score(vec![1, 2, 1, 2]));
        assert_eq!(8, calc_scenic_score(vec![2, 2, 2, 1]));
    }
}
