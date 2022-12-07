use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process::exit,
    vec::Vec,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("cloud not read line"))
        .collect()
}

fn referee(play: String) -> i32 {
    let picks: Vec<&str> = play.split_whitespace().collect();
    let elf_pick: &str = picks[0];
    let player_pick: &str = picks[1];
    let mut points: i32 = 0;

    if player_pick == "X" {
        points += 1;
        match elf_pick {
            "A" => points += 3, // d
            "B" => points += 0, // l
            "C" => points += 6, // w
            &_ => {
                eprintln!("Invalid elf pick!");
                exit(1);
            }
        }
    } else if player_pick == "Y" {
        points += 2;
        match elf_pick {
            "A" => points += 6, // w
            "B" => points += 3, // d
            "C" => points += 0, // l
            &_ => {
                eprintln!("Invalid elf pick!");
                exit(1);
            }
        }
    } else if player_pick == "Z" {
        points += 3;
        match elf_pick {
            "A" => points += 0, // l
            "B" => points += 6, // w
            "C" => points += 3, // d
            &_ => {
                eprintln!("Invalid elf pick!");
                exit(1);
            }
        }
    } else {
        eprintln!("Invalid player pick!");
        exit(1);
    }
    return points;
}

fn main() {
    let strategies = lines_from_file("./day_2/day_2.in");
    let mut player_score: i32 = 0;

    for play in strategies {
        player_score += referee(play);
    }

    println!("Player points: {}", player_score)
}
