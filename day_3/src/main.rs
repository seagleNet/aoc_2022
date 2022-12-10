use file_utils::lines_from_file;

fn split_rucksack(rucksack: &str) -> (&str, &str) {
    rucksack.split_at(rucksack.len() / 2)
}

fn return_duplicate(comp_one: &str, comp_two: &str) -> char {
    for item in comp_one.chars() {
        if comp_two.contains(item) {
            return item;
        }
    }
    panic!("no duplicates")
}

fn return_duplicates(comp_one: &str, comp_two: &str) -> String {
    let mut duplicates: String = String::new();

    for item in comp_one.chars() {
        if comp_two.contains(item) {
            duplicates.push(item);
        }
    }
    duplicates
}

fn group_rucksacks(rucksacks: Vec<String>) -> Vec<Vec<String>> {
    let mut group_of_rucksacks: Vec<Vec<String>> = vec![];
    let mut current_group: Vec<String> = Vec::new();

    for rucksack in rucksacks {
        current_group.push(rucksack);
        if current_group.len() == 3 {
            group_of_rucksacks.push(current_group);
            current_group = Vec::new();
        }
    }
    group_of_rucksacks
}

fn map_to_priority(item: char) -> u8 {
    let char_as_num = item as u8;
    if item.is_ascii_lowercase() {
        return char_as_num - 'a' as u8 + 1;
    } else if item.is_ascii_uppercase() {
        return char_as_num - 'A' as u8 + 27;
    }
    panic!("not ascii char")
}

fn main() {
    let rucksacks = lines_from_file("./day_3/day_3.in");
    let mut prio_total: i32 = 0;
    let mut prio_total_pt2: i32 = 0;

    for rucksack in rucksacks.clone() {
        let (comp_one, comp_two) = split_rucksack(rucksack.as_str());
        prio_total += map_to_priority(return_duplicate(comp_one, comp_two)) as i32;
    }

    let groups = group_rucksacks(rucksacks);
    for group in groups {
        let duplicates_one = return_duplicates(group[0].as_str(), group[1].as_str());
        let duplicates = return_duplicate(duplicates_one.as_str(), group[2].as_str());
        prio_total_pt2 += map_to_priority(duplicates) as i32;
    }

    println!("Priority total: {}", prio_total);
    println!("Priority total pt2: {}", prio_total_pt2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_split() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let (left, right) = split_rucksack(rucksack);
        assert_eq!("vJrwpWtwJgWr", left);
        assert_eq!("hcsFMMfFFhFp", right);
    }

    #[test]
    fn test_return_duplicate() {
        let duplicate: char = return_duplicate("asdf", "jklf");
        assert_eq!('f', duplicate);
    }

    #[test]
    fn test_prio_map() {
        assert_eq!(1, map_to_priority('a'));
        assert_eq!(26, map_to_priority('z'));
        assert_eq!(27, map_to_priority('A'));
        assert_eq!(52, map_to_priority('Z'));
    }

    #[test]
    fn test_return_duplicates() {
        let result = return_duplicates("vJrwpWtwJgWr", "hcsFMMfFFhFp");
        assert_eq!(vec![])
    }
}
