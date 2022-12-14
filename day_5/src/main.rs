use file_utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;

fn get_crates<'a>(line: &'a String) -> regex::Captures<'a> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?:( {3}|\[[A-Z]\]) ?)+").unwrap();
    }
    RE.captures(&line).unwrap()
}

fn get_init_state(lines: Vec<String>) -> Vec<String> {
    let mut input: Vec<String> = Vec::new();
    let mut i: usize = 0;

    while lines.iter().len() != 0 {
        let test = get_crates(lines.first().unwrap());
        println!("{:?}", test);
        input.push(String::new());
        let caps = lines
            .iter()
            .map(|line| get_crates(line))
            .map(|caps| caps.get(0))
            .map(|caps| caps.unwrap().as_str().parse::<char>().unwrap());

        for cap in caps {
            input[i].push(cap);
        }
        i += 1;
    }
    return input
}

fn main() {
    let lines = lines_from_file("./day_5/day_5.in");
    get_init_state(lines);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::get_init_state;

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

        let init_state = get_init_state(lines);

        println!("{:?}", init_state);
    }

    #[test]
    fn test_string() {
        let mut test = String::new();

        test.push('a');
        test.push('b');
        test.push('c');
        assert_eq!("abc", test);

        let mut test_vec: Vec<String> = Vec::new();

        test_vec.push(String::new());
        test_vec[0].push('a');
        test_vec[0].push('b');
        test_vec.push(String::new());
        test_vec[1].push('x');
        test_vec[1].push('y');
        assert_eq!(vec!["ab", "xy"], test_vec);
    }
}
