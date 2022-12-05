use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap_or_default().parse::<i32>().unwrap_or_default())
        .collect()
}

fn main() {
    let cal_in = lines_from_file("./day_1.in");
    let mut cal_per_elf: Vec<i32> = Vec::new();
    let mut cal_current: i32 = 0;
    let mut cal_highest: &i32 = &0;
    let mut elf_highest: usize = 0;

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

    for (_i, cal_elf_current) in cal_per_elf.iter().enumerate() {
        if cal_elf_current > &cal_highest {
            cal_highest = cal_elf_current;
            elf_highest = _i;
        }
    }

    println!(
        "* Elf {} is carrying the most cal: {}",
        elf_highest, cal_highest
    )
}
