use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    vec::Vec,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("cloud not read line"))
        .collect()
}

fn main() {
    let strategies = lines_from_file("./day_2/day_2.in");

    for (_i, picks) in strategies.iter().enumerate() {
        println!("{picks}");
        //let elf_pick: String = picks.split(" ").nth(1).map(String::from);
        //let player_pick: String = picks.split(" ").nth(2).map(String::from);
    }
}
