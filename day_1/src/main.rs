use file_utils::lines_from_file;

fn main() {
    let cal_in = lines_from_file("./day_1/day_1.in");
    let mut cal_per_elf: Vec<i32> = Vec::new();
    let mut cal_current: i32 = 0;
    let mut cal_top_three: i32 = 0;

    for cal in cal_in {
        if !cal.is_empty() {
            cal_current += cal.parse::<i32>().unwrap();
        } else {
            cal_per_elf.push(cal_current);
            cal_current = 0;
        }
    }
    cal_per_elf.push(cal_current);

    for (_i, elf) in cal_per_elf.iter().enumerate() {
        println!("Elf {}, calories {}", _i, elf);
    }

    cal_per_elf.sort();
    cal_per_elf.reverse();

    println!("***");

    for _i in 0..3 {
        println!("{}. {}", _i + 1, cal_per_elf[_i]);
        cal_top_three += cal_per_elf[_i];
    }
    println!("Top 3 total: {}", cal_top_three);
}
