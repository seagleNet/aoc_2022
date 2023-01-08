use file_utils::lines_from_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn parse_command(line: String) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^\$ (cd|ls)").unwrap();
    }

    if let Some(captures) = RE.captures(line.as_str()) {
        if let Some(m) = captures.get(1) {
            return m.as_str().to_string();
        }
    }
    panic!("couldn't parse command")
}

fn parse_cd(line: &String, pwd: &mut Vec<String>) {
    let groups: Vec<&str> = line.split_whitespace().collect();
    let dir: String = groups[2].to_string();

    if dir == "/" {
        pwd.clear();
        pwd.push(dir);
    } else if dir == ".." {
        if pwd.len() > 1 {
            pwd.pop();
        }
    } else {
        let mut input = dir;
        input.push('/');
        pwd.push(input);
    }
}

fn vec_to_string(input: &Vec<String>) -> String {
    let mut str: String = String::new();

    for i in input {
        str.push_str(i);
    }
    str
}

fn parse_ls(
    lines: &Vec<String>,
    line_index: usize,
    directories: &mut HashMap<String, i32>,
    pwd: &Vec<String>,
) {
    let start: usize = line_index + 1;
    let mut dir_size = 0;

    for line in &lines[start..] {
        let groups: Vec<&str> = line.split_whitespace().collect();
        if groups[0].parse::<i32>().is_ok() {
            dir_size += groups[0].parse::<i32>().unwrap();
        } else if groups[0] == "$" {
            directories.insert(vec_to_string(pwd), dir_size);
            break;
        }
    }
    directories.insert(vec_to_string(pwd), dir_size);
}

fn parse_input(lines: Vec<String>) -> HashMap<String, i32> {
    let mut directories: HashMap<String, i32> = HashMap::new();
    let mut pwd: Vec<String> = Vec::new();
    let mut line_index: usize = 0;

    for line in lines.iter() {
        if line.starts_with("$") {
            match parse_command(line.clone()).as_str() {
                "cd" => parse_cd(line, &mut pwd),
                "ls" => parse_ls(&lines, line_index, &mut directories, &pwd),
                _ => panic!("command not found"),
            }
        }
        line_index += 1;
    }
    directories
}

fn get_recursive_sizes(directories: HashMap<String, i32>) -> HashMap<String, i32> {
    let mut recursive_sizes: HashMap<String, i32> = HashMap::new();

    for dir in &directories {
        let mut r_size = 0;
        for d in &directories {
            if d.0.starts_with(dir.0) {
                r_size += d.1;
            }
        }
        recursive_sizes.insert(dir.0.to_string(), r_size);
    }
    recursive_sizes
}

fn pt1(lines: Vec<String>) -> i32 {
    let directories = parse_input(lines);
    let recursive_sizes = get_recursive_sizes(directories);
    let mut total_size = 0;

    for size in recursive_sizes {
        if size.1 <= 100000 {
            total_size += size.1;
        }
    }
    total_size
}

fn pt2(lines: Vec<String>) -> i32 {
    let directories = parse_input(lines);
    let recursive_sizes = get_recursive_sizes(directories);
    let disk_free = 70000000 - recursive_sizes.get("/").unwrap();
    let mut del_selection: Vec<i32> = Vec::new();

    for size in recursive_sizes {
        if size.1 + disk_free >= 30000000 {
            del_selection.push(size.1);
        }
    }
    *del_selection.iter().min().unwrap()
}

fn main() {
    let lines = lines_from_file("./day_7/input.txt");

    println!("result pt1: {}", pt1(lines.clone()));
    println!("result pt2: {}", pt2(lines));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines: Vec<String> = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];

        println!("{:?}", pt1(lines));
    }

    #[test]
    fn test_parse_command() {
        assert_eq!("cd".to_string(), parse_command("$ cd /".to_string()));
        assert_eq!("ls".to_string(), parse_command("$ ls".to_string()));
    }

    #[test]
    fn test_parse_cd() {
        let lines: Vec<String> = vec![
            "$ cd /".to_string(),
            "$ cd a".to_string(),
            "$ cd e".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
        ];
        let mut pwd: Vec<String> = Vec::new();

        for line in lines {
            parse_cd(&line, &mut pwd);
        }

        println!("{:?}", pwd);
    }

    #[test]
    fn test_vec_to_string() {
        let vec = vec!["/", "a/", "b/"];
        let mut str: String = String::new();
        for v in vec {
            str.push_str(v);
        }
        assert_eq!("/a/b/".to_string(), str);
    }
}
