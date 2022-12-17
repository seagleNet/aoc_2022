use file_utils::lines_from_file;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn get_marker_position(input: &mut VecDeque<char>) -> i32 {
    let mut marker_position = 4;
    let mut segment: VecDeque<char> = VecDeque::new();

    for _ in 0..4 {
        segment.push_back(input.pop_front().unwrap());
    }
    for _ in input.clone() {
        if has_unique_elements(&segment) {
            return marker_position;
        } else {
            segment.pop_front();
            segment.push_back(input.pop_front().unwrap());
            marker_position += 1;
        }
    }
    panic!()
}

fn pt1(lines: Vec<String>) -> i32 {
    let mut marker_position: i32 = 0;

    for line in lines {
        let mut input = line.chars().collect::<VecDeque<char>>();
        marker_position += get_marker_position(&mut input);
    }
    marker_position
}

fn main() {
    let lines = lines_from_file("./day_6/day_6.in");

    println!("result pt1: {}", pt1(lines.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_state() {
        let lines: Vec<String> = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),
        ];
        let mut marker_position: i32 = 0;

        for line in lines {
            let mut input = line.chars().collect::<VecDeque<char>>();
            marker_position += get_marker_position(&mut input);
        }

        println!("{:?}", marker_position);
    }
}
