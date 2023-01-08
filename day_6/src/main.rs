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

fn get_marker_position(input: &mut VecDeque<char>, buffer_size: i32) -> i32 {
    let mut marker_position = buffer_size;
    let mut segment: VecDeque<char> = VecDeque::new();

    for _ in 0..buffer_size {
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
        marker_position += get_marker_position(&mut input, 4);
    }
    marker_position
}

fn pt2(lines: Vec<String>) -> i32 {
    let mut marker_position: i32 = 0;

    for line in lines {
        let mut input = line.chars().collect::<VecDeque<char>>();
        marker_position += get_marker_position(&mut input, 14);
    }
    marker_position
}

fn main() {
    let lines = lines_from_file("./day_6/input.txt");

    println!("result pt1: {}", pt1(lines.clone()));
    println!("result pt2: {}", pt2(lines.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maker_position() {
        let lines: Vec<String> = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),
        ];
        let mut marker_position: i32 = 0;

        for line in lines {
            let mut input = line.chars().collect::<VecDeque<char>>();
            marker_position += get_marker_position(&mut input, 4);
        }

        println!("{:?}", marker_position);
    }
}
