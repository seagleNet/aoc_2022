use file_utils::lines_from_file;

fn get_pair_ranges(pair: &str) -> (Vec<i32>, Vec<i32>) {
    let (left, right) = pair.split_once(',').unwrap();
    (
        left.split('-')
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
        right
            .split('-')
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>(),
    )
}

fn compare_ranges(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let range_l = *left.iter().min().unwrap()..=*left.iter().max().unwrap();
    let range_r = *right.iter().min().unwrap()..=*right.iter().max().unwrap();

    if range_l.contains(&right[0]) && range_l.contains(&right[1]) {
        return 1;
    } else if range_r.contains(&left[0]) && range_r.contains(&left[1]) {
        return 1;
    } else {
        return 0;
    }
}

fn compare_ranges_pt2(left: Vec<i32>, right: Vec<i32>) -> i32 {
    let range_l = *left.iter().min().unwrap()..=*left.iter().max().unwrap();
    let range_r = *right.iter().min().unwrap()..=*right.iter().max().unwrap();

    if range_l.contains(&right[0]) || range_l.contains(&right[1]) {
        return 1;
    } else if range_r.contains(&left[0]) || range_r.contains(&left[1]) {
        return 1;
    } else {
        return 0;
    }
}

fn main() {
    let pairs = lines_from_file("./day_4/input.txt");

    let result = pairs
        .clone()
        .iter()
        .map(|pair| get_pair_ranges(pair))
        .map(|ranges| compare_ranges(ranges.0, ranges.1))
        .sum::<i32>();
    println!("Total fully contained: {}", result);

    let result_pt2 = pairs
        .iter()
        .map(|pair| get_pair_ranges(pair))
        .map(|ranges| compare_ranges_pt2(ranges.0, ranges.1))
        .sum::<i32>();
    println!("Total overlapped: {}", result_pt2);
}

#[cfg(test)]
mod tests {
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
            .sum();
        assert_eq!(1, result);
    }
}
