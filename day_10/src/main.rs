use file_utils::lines_from_file;
use std::collections::HashMap;

fn parse_instructions(line: String) -> (String, i32) {
    let result: Vec<&str> = line.split_whitespace().collect();

    if result[0].contains("addx") {
        (
            result[0].try_into().unwrap(),
            result[1].parse::<i32>().unwrap(),
        )
    } else if result[0].contains("noop") {
        (result[0].try_into().unwrap(), 0)
    } else {
        panic!("could not parse line: {:?}", line)
    }
}

fn pt1(lines: Vec<String>) -> i32 {
    let mut x = 1;
    let mut cycle = 1;
    let mut signals: HashMap<i32, i32> = HashMap::new();

    for line in lines {
        let instruction = parse_instructions(line);

        if instruction.0.contains("addx") {
            for cycle_run in 0..2 {
                cycle += 1;
                if cycle_run == 1 {
                    x += instruction.1;
                }
                if (cycle - 20) % 40 == 0 {
                    signals.insert(cycle, cycle * x);
                }
            }
        } else if instruction.0.contains("noop") {
            cycle += 1;
            if (cycle - 20) % 40 == 0 {
                signals.insert(cycle, cycle * x);
            }
        }
    }

    signals.values().sum()
}

fn draw_pixel(cycle: i32, x: i32) -> char {
    let sprite = x - 1..=x + 1;
    if sprite.contains(&cycle) {
        '#'
    } else {
        '.'
    }
}

fn pt2(lines: Vec<String>) {
    let mut x = 1;
    let mut cycle = 0;
    let mut crt_lines: Vec<String> = Vec::new();
    let mut crt_line_current = 0;

    crt_lines.push(String::new());

    for line in lines {
        let instruction = parse_instructions(line);

        if instruction.0.contains("addx") {
            for cycle_run in 0..2 {
                crt_lines[crt_line_current].push(draw_pixel(cycle, x));
                cycle += 1;
                if cycle_run == 1 {
                    x += instruction.1;
                }
                if crt_lines[crt_line_current].len() == 40 {
                    cycle = 0;
                    crt_line_current += 1;
                    crt_lines.push(String::new());
                }
            }
        } else if instruction.0.contains("noop") {
            crt_lines[crt_line_current].push(draw_pixel(cycle, x));
            cycle += 1;
            if crt_lines[crt_line_current].len() == 40 {
                cycle = 0;
                crt_line_current += 1;
                crt_lines.push(String::new());
            }
        }
    }

    for crt_line in crt_lines {
        println!("{}", crt_line)
    }
}

fn main() {
    let lines = lines_from_file("./day_10/input.txt");

    println!("result pt1: {}", pt1(lines.clone()));
    println!("result pt2:");
    pt2(lines);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines = lines_from_file("./example.txt");

        let result_pt1 = pt1(lines.clone());
        println!("{:?}", result_pt1);
        assert_eq!(13140, result_pt1);
    }

    #[test]
    fn test_pt2() {
        let lines = lines_from_file("./example.txt");

        pt2(lines.clone())
    }

    #[test]
    fn test_parse_instructions() {
        assert_eq!(
            ("addx".to_string(), 15),
            parse_instructions("addx 15".to_string())
        );
        assert_eq!(
            ("addx".to_string(), -11),
            parse_instructions("addx -11".to_string())
        );
        assert_eq!(
            ("noop".to_string(), 0),
            parse_instructions("noop".to_string())
        );
    }

    #[test]
    fn test_modulo() {
        assert_eq!(-15, (5 - 20) % 40);
        assert_eq!(0, (20 - 20) % 40);
        assert_eq!(12, (32 - 20) % 40);
        assert_eq!(0, (60 - 20) % 40);
        assert_eq!(0, (100 - 20) % 40);
        assert_eq!(20, (80 - 20) % 40);
    }
}
