use file_utils::lines_from_file;

fn coordinates(lines: &Vec<Vec<char>>, query: char) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    for line in lines {
        for ch in line {
            if ch == &query {
                return (x, y);
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }
    panic!("could not find {}", query)
}

fn pt1(lines: Vec<Vec<char>>) -> i32 {
    return 31;
}

fn main() {
    let lines: Vec<Vec<char>> = lines_from_file("./day_11/input.txt")
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

    println!("result pt1: {}", pt1(lines.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let lines = lines_from_file("./example.txt");
    }

    #[test]
    fn test_coordinates() {
        let lines: Vec<Vec<char>> = lines_from_file("./example.txt")
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

        assert_eq!((0, 0), coordinates(&lines, 'S'));
        assert_eq!((5, 2), coordinates(&lines, 'E'));
    }
}
