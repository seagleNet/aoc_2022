use file_utils::lines_from_file;
use std::collections::HashMap;

fn parse_instructions(line: String) -> (char, i32) {
    let result: Vec<&str> = line.split_whitespace().collect();

    if result.len() == 2 {
        (
            result[0].parse::<char>().unwrap(),
            result[1].parse::<i32>().unwrap(),
        )
    } else {
        panic!("could not parse line: {:?}", line)
    }
}

fn move_head(head_position: (i32, i32), mv_direction: char) -> (i32, i32) {
    let x = head_position.0;
    let y = head_position.1;

    match mv_direction {
        'U' => (x, y + 1),
        'D' => (x, y - 1),
        'L' => (x - 1, y),
        'R' => (x + 1, y),
        _ => panic!("could not parse direction"),
    }
}

fn adjacency(head_position: (i32, i32), tail_position: (i32, i32)) -> bool {
    let delta_x = head_position.0 - tail_position.0;
    let delta_y = head_position.1 - tail_position.1;
    let valid_range = -1..=1;

    if valid_range.contains(&delta_x) && valid_range.contains(&delta_y) {
        true
    } else {
        false
    }
}

fn move_tail(head_position: (i32, i32), tail_position: (i32, i32)) -> (i32, i32) {
    let t_x = tail_position.0;
    let t_y = tail_position.1;
    let delta_x = head_position.0 - tail_position.0;
    let delta_y = head_position.1 - tail_position.1;
    let mut delta_x_norm = delta_x;
    let mut delta_y_norm = delta_y;

    if delta_x.is_negative() {
        delta_x_norm = -delta_x;
    }
    if delta_y.is_negative() {
        delta_y_norm = -delta_y;
    }

    if delta_x == 0 {
        return (t_x, t_y + delta_y / delta_y_norm);
    } else if delta_y == 0 {
        return (t_x + delta_x / delta_x_norm, t_y);
    } else {
        return (t_x + delta_x / delta_x_norm, t_y + delta_y / delta_y_norm);
    }
}

fn pt1(lines: Vec<String>) -> i32 {
    let mut head_position = (0, 0);
    let mut tail_position = (0, 0);
    let mut tail_position_log: HashMap<(i32, i32), i32> = HashMap::new();

    tail_position_log.insert(tail_position, 1);

    for line in lines {
        let instruction = parse_instructions(line);
        let mv_direction = instruction.0;
        let mv_count = instruction.1;

        for _ in 0..mv_count {
            head_position = move_head(head_position, mv_direction);
            if !adjacency(head_position, tail_position) {
                tail_position = move_tail(head_position, tail_position);
                tail_position_log
                    .entry(tail_position)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }

    tail_position_log.keys().len().try_into().unwrap()
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

    #[test]
    fn test_parse_instructions() {
        assert_eq!(('R', 4), parse_instructions("R 4".to_string()));
        assert_eq!(('U', 4), parse_instructions("U 4".to_string()));
        assert_eq!(('L', 3), parse_instructions("L 3".to_string()));
    }

    #[test]
    fn test_move_head() {
        let mut head_position = (0, 0);

        for _ in 0..4 {
            head_position = move_head(head_position, 'R');
        }
        assert_eq!((4, 0), head_position);

        for _ in 0..4 {
            head_position = move_head(head_position, 'U');
        }
        assert_eq!((4, 4), head_position);
    }

    #[test]
    fn test_move_tail() {
        assert_eq!((1, 0), move_tail((2, 0), (0, 0)));
        assert_eq!((4, 3), move_tail((4, 2), (5, 4)));
    }

    #[test]
    fn test_check_adjacency() {
        assert_eq!(true, adjacency((0, 0), (1, 1)));
        assert_eq!(true, adjacency((2, 3), (1, 2)));
        assert_eq!(true, adjacency((2, 3), (3, 4)));
        assert_eq!(false, adjacency((2, 3), (1, 1)));
        assert_eq!(false, adjacency((2, 3), (4, 1)));
    }

    #[test]
    fn test_neg() {
        let a = -2;

        assert_eq!(-a, 2);

        let b = 2;
        assert_eq!(-b, -2);
    }
}
