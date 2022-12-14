use file_utils::lines_from_file;

fn main() {
    let lines = lines_from_file("./day_5/day_5.in");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_initial_state() {
        let lines = vec![
            "    [D]    ",
            "[N] [C]    ",
            "[Z] [M] [P]",
            " 1   2   3 ",
            "",
            "move 1 from 2 to 1",
            "move 3 from 1 to 3",
        ];
        let mut input: Vec<Vec<&str>> = Vec::new();

        while lines.iter().len() != 0 {
            for (i, line) in lines.iter().enumerate() {
                input[i].push(&line.chars().nth(2).unwrap().to_string());

            }
            lines.iter().map(|line| line.chars().nth(3)).iter;
        }
    }
}
