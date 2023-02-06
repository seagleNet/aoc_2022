use file_utils::lines_from_file;
use pathfinding::prelude::astar;

fn get_height(char: char) -> i8 {
    if char.is_ascii_lowercase() {
        return (char as u8 - 'a' as u8 + 1) as i8;
    } else if char == 'S' {
        return 1;
    } else if char == 'E' {
        return 26;
    } else {
        panic!("not ascii char: {:?}", char)
    }
}

fn is_traversable(curr: char, next: char) -> bool {
    let height_diff = get_height(next) - get_height(curr);

    if (std::i8::MIN..=1).contains(&height_diff) {
        return true;
    } else {
        return false;
    }
}

fn coordinates(lines: &Vec<Vec<char>>, query: char) -> (i32, i32) {
    let mut x = 0;
    let mut y = 0;

    for line in lines {
        for ch in line {
            if ch == &query {
                return (x, y);
            }
            y += 1;
        }
        y = 0;
        x += 1;
    }
    panic!("could not find {}", query)
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn distance(&self, other: &Pos) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }

    fn successors(&self, grid: &Vec<Vec<char>>) -> Vec<(Pos, u32)> {
        let &Pos(x, y) = self;
        let mut result: Vec<(Pos, u32)> = Vec::new();

        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if (dx == 0 && dy != 0) || (dx != 0 && dy == 0) {
                    let next_x = self.0 as i32 + dx;
                    let next_y = self.1 as i32 + dy;
                    if next_x >= 0
                        && next_x < grid.len() as i32
                        && next_y >= 0
                        && next_y < grid[0].len() as i32
                    {
                        let curr_h = grid[self.0 as usize][self.1 as usize];
                        let next_h = grid[next_x as usize][next_y as usize];
                        if is_traversable(curr_h, next_h) {
                            result.push((Pos(x + dx, y + dy), 1));
                        }
                    }
                }
            }
        }

        return result;
    }
}

fn pt1(grid: Vec<Vec<char>>) -> u32 {
    let start = coordinates(&grid, 'S');
    let end = coordinates(&grid, 'E');
    let goal: Pos = Pos(end.0, end.1);

    let result = astar(
        &Pos(start.0, start.1),
        |p| p.successors(&grid),
        |p| p.distance(&goal),
        |p| *p == goal,
    );
    result.expect("no path found").1
}

fn pt2(grid: Vec<Vec<char>>) -> u32 {
    let mut start_candidates: Vec<Pos> = Vec::new();
    let end = coordinates(&grid, 'E');
    let goal: Pos = Pos(end.0, end.1);
    let mut distances: Vec<u32> = Vec::new();

    for x in grid.iter().enumerate() {
        for y in x.1.iter().enumerate() {
            if y.1 == &'a' {
                start_candidates.push(Pos(x.0 as i32, y.0 as i32))
            }
        }
    }

    for start in start_candidates {
        if let Some(distance) = astar(
            &Pos(start.0, start.1),
            |p| p.successors(&grid),
            |p| p.distance(&goal),
            |p| *p == goal,
        ) {
            distances.push(distance.1);
        }
    }

    let result = distances.iter().min();
    *result.expect("no result")
}

fn main() {
    let grid: Vec<Vec<char>> = lines_from_file("./day_12/input.txt")
        .into_iter()
        .map(|s| s.chars().collect())
        .collect();

    println!("result pt1: {}", pt1(grid.clone()));
    println!("result pt2: {}", pt2(grid.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let grid: Vec<Vec<char>> = lines_from_file("./example.txt")
            // let grid: Vec<Vec<char>> = lines_from_file("./day_12/example.txt")
            .into_iter()
            .map(|s| s.chars().collect())
            .collect();

        println!("{:?}", grid);
        assert_eq!(31, pt1(grid));
    }

    #[test]
    fn test_coordinates() {
        let grid: Vec<Vec<char>> = lines_from_file("./example.txt")
            .into_iter()
            .map(|s| s.chars().collect())
            .collect();

        assert_eq!((0, 0), coordinates(&grid, 'S'));
        assert_eq!((5, 2), coordinates(&grid, 'E'));
    }

    #[test]
    fn test_get_height() {
        assert_eq!(1, get_height('S'));
        assert_eq!(1, get_height('a'));
        assert_eq!(26, get_height('E'));
        assert_eq!(26, get_height('z'));
    }

    #[test]
    fn test_is_traversable() {
        assert_eq!(true, is_traversable('a', 'b'));
        assert_eq!(true, is_traversable('a', 'a'));
        assert_eq!(true, is_traversable('b', 'a'));
        assert_eq!(true, is_traversable('z', 'a'));
        assert_eq!(false, is_traversable('a', 'c'));
        assert_eq!(false, is_traversable('a', 'z'));
    }
}
