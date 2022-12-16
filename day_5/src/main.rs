use file_utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;

fn parse_init_state(line: &str, stacks: &mut Vec<VecDeque<char>>) {
    let mut input = line.to_string();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\[[A-Z]\] ?| {4})").unwrap();
    }

    let mut group_index = 0;
    while let Some(captures) = RE.captures(&input.as_str()) {
        if let Some(m) = captures.get(1) {
            if stacks.get(group_index).is_none() {
                stacks.push(VecDeque::new());
            }
            let char = m.as_str().chars().nth(1).unwrap();
            if char.is_alphabetic() {
                stacks[group_index].push_front(char);
            }
            input.replace_range(m.range(), "");
        }
        group_index += 1;
    }
}

fn parse_instruction(line: &str, instructions: &mut Vec<Vec<i32>>) {
    let input = line.to_string();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    }

    if let Some(captures) = RE.captures(&input.as_str()) {
        let index = instructions.len();
        instructions.push(Vec::new());
        for i in 1..=3 {
            if let Some(m) = captures.get(i) {
                let int = m.as_str().parse::<i32>().unwrap();
                if int.is_positive() {
                    instructions[index].push(int);
                }
            }
        }
    }
}

fn exec_instruction(instruction: Vec<i32>, stacks: &mut Vec<VecDeque<char>>) {
    let mv_count = instruction[0].try_into().unwrap();
    let src = instruction[1].try_into().map(|src: usize | src -1 ).unwrap();
    let dst = instruction[2].try_into().map(|src: usize | src -1 ).unwrap();

    for _ in 0..mv_count {
        let transport = stacks[src].pop_back().unwrap();
        stacks[dst].push_back(transport);
    }
}

fn exec_instruction_pt2(instruction: Vec<i32>, stacks: &mut Vec<VecDeque<char>>) {
    let mv_count = instruction[0].try_into().unwrap();
    let src = instruction[1].try_into().map(|src: usize | src -1 ).unwrap();
    let dst = instruction[2].try_into().map(|src: usize | src -1 ).unwrap();
    let mut transport: Vec<char> = Vec::new();

    for _ in 0..mv_count {
        let load = stacks[src].pop_back().unwrap();
        transport.push(load);
    }
    while !transport.is_empty() {
        let unload = transport.pop().unwrap();
        stacks[dst].push_back(unload);
    }
}

fn pt1(lines: Vec<String>) -> String {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut instructions: Vec<Vec<i32>> = Vec::new();
    let mut top_crates = String::new();

    for line in lines.iter() {
        parse_init_state(line, &mut stacks);
        parse_instruction(line, &mut instructions);
    }

    for instruction in instructions.clone() {
        exec_instruction(instruction, &mut stacks);
    }

    for stack in stacks {
        top_crates.push(stack[stack.len() - 1]);
    }

    top_crates
}

fn pt2(lines: Vec<String>) -> String {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let mut instructions: Vec<Vec<i32>> = Vec::new();
    let mut top_crates = String::new();

    for line in lines.iter() {
        parse_init_state(line, &mut stacks);
        parse_instruction(line, &mut instructions);
    }

    for instruction in instructions {
        exec_instruction_pt2(instruction, &mut stacks);
    }

    for stack in stacks {
        top_crates.push(stack[stack.len() - 1]);
    }

    top_crates
}

fn main() {
    let lines = lines_from_file("./day_5/day_5.in");

    println!("result pt1: {}", pt1(lines.clone()));

    println!("result pt2: {}", pt2(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_state() {
        let lines: Vec<String> = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 2 to 3".to_string(),
        ];
        let mut stacks: Vec<VecDeque<char>> = Vec::new();

        for line in lines.iter() {
            parse_init_state(line, &mut stacks);
        }
        println!("{:?}", stacks);
    }

    #[test]
    fn test_parse_instruction() {
        let lines: Vec<String> = vec![
            "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
        ];
        let mut instructions: Vec<Vec<i32>> = Vec::new();

        for line in lines.iter() {
            parse_instruction(line, &mut instructions);
        }
        println!("{:?}", instructions);
    }
}
