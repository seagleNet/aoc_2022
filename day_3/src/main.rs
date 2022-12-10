use std::marker::Tuple;

use file_utils::lines_from_file;

fn split_rucksack(rucksack: &str) {
    rucksack.split_at(rucksack.len() / 2 + 1);
}

fn calculate_priorities(rucksack: String) -> i32 {
    let mut prio_per_rucksack = 0;

    println!("{compartment_one} {compartment_two}");

    for item in compartment_one.chars() {}
    return 0;
}

fn main() {
    let rucksacks = lines_from_file("./day_3/day_3.in");
    let mut prio_total = 0;

    for rucksack in rucksacks {
        prio_total += calculate_priorities(rucksack);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rucksack_split() {
        let rucksack = "vJrwpWtwJgWrhcsFMMfFFhFp";
        let result = split_rucksack(rucksack);
        assert!(result.is_ok());
    }
}
