use file_utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;

fn parse_single_line(line: &str, stacks: &mut Vec<Vec<char>>) {
    let mut input = line.to_string();
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\[[A-Z]\]|   )").unwrap();
    }

    let mut group_index = 0;
    while let Some(captures) = RE.captures(&input.as_str()) {
        if let Some(m) = captures.get(1) {
            if stacks.get(group_index).is_none() {
                stacks.push(vec![]);
            }
            println!("{} group is '{}'", group_index, m.as_str());
            let char = m.as_str().chars().nth(1).unwrap();
            if char.is_alphabetic() {
                stacks[group_index].push(char);
            }
            input.replace_range(m.range(), "");
        }
        group_index += 1;
    }
}

fn main() {
    let lines = lines_from_file("./day_5/day_5.in");
    let mut stacks: Vec<Vec<char>> = vec![];

    for line in lines.iter() {
        parse_single_line(line, &mut stacks);
    }

    println!("parsed vecs: {:?}", stacks);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_state() {
        let lines: Vec<String> = vec![
            // "    [D]    ".to_string(),
            "[N] [C]    ".to_string(),
            "[Z] [M] [P]".to_string(),
            " 1   2   3 ".to_string(),
            "".to_string(),
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
        ];
        let mut stacks: Vec<Vec<char>> = vec![];

        for line in lines.iter() {
            parse_single_line(line, &mut stacks);
        }

        println!("{:?}", stacks);
    }
}
