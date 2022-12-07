use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    vec::Vec,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap_or_default().parse::<i32>().unwrap_or_default())
        .collect()
}

fn main() {
    let cal_in = lines_from_file("./day_1/day_1.in");
    let mut cal_per_elf: Vec<i32> = Vec::new();
    let mut cal_current: i32 = 0;
    let mut cal_top_three: i32 = 0;

    for cal in cal_in {
        if cal > 0 {
            cal_current = cal_current + cal;
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
        cal_top_three = cal_top_three + cal_per_elf[_i];
    }
    println!("Top 3 total: {}", cal_top_three);
}
