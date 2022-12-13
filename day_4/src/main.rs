use file_utils::lines_from_file;
use std::str::FromStr;

fn get_pair_ranges(pair: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right) = pair.split_once(',').unwrap();
    (
        left.split('-')
            .filter_map(|v| i32::from_str(v).ok())
            .collect::<Vec<i32>>(),
        right
            .split('-')
            .filter_map(|v| i32::from_str(v).ok())
            .collect::<Vec<i32>>(),
    )
}

fn compare_ranges(left: Vec<i32>, right: Vec<i32>) -> i32 {
    if left.iter().min() >= right.iter().min() && left.iter().max() <= right.iter().max() {
        return 1;
    } else if right.iter().min() >= left.iter().min() && right.iter().max() <= left.iter().max() {
        return 1;
    } else {
        return 0;
    }
}

fn main() {
    let pairs = lines_from_file("./day_4/day_4.in");

    let result = pairs
            .iter()
            .map(|pair| get_pair_ranges(pair))
            .map(|ranges| compare_ranges(ranges.0, ranges.1))
            .fold(0, |current, v| current + v);
    println!("Total fully contained: {}", result);
}

#[cfg(test)]
mod tests {
    use std::result;

    use super::*;

    #[test]
    fn test_split_pair() {
        let pair: &str = "2-4,6-8";
        let result = pair.split(",").collect::<Vec<&str>>();
        assert_eq!("2-4", result[0]);
        assert_eq!("6-8", result[1]);
    }

    #[test]
    fn test_split_pair_v2() {
        let pair: &str = "2-4,6-8";
        assert_eq!(pair.split_once(','), Some(("2-4", "6-8")));
        let (left, right) = pair.split_once(',').unwrap();
        assert_eq!("2-4", left);
        assert_eq!("6-8", right);
    }

    #[test]
    fn test_split_pair_v3() {
        let pair: &str = "2-4,6-8";
        let (left, right) = get_pair_ranges(pair);
        assert_eq!(vec![2, 4], left);
        assert_eq!(vec![6, 8], right);
    }

    #[test]
    fn test_full() {
        let pairs = vec!["2-4,6-8", "3-7,4-7"];
        let result = pairs
            .iter()
            .map(|pair| get_pair_ranges(pair))
            .map(|ranges| compare_ranges(ranges.0, ranges.1))
            .fold(0, |current, v| current + v);
        assert_eq!(1, result);
    }
}
