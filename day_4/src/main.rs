use file_utils::lines_from_file;

fn get_pair_ranges(pair: &str) -> Vec<&str> {
    pair.split(",").collect::<Vec<&str>>()
}

fn main() {
    let pairs = lines_from_file("./day_4/day_4.in");
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
}
