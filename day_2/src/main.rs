use std::process::exit;
use file_utils::lines_from_file;

fn referee(elf_pick: &str, player_pick: &str) -> i32 {
    let mut points: i32 = 0;

    if player_pick == "P_A" {
        points += 1;
        match elf_pick {
            "A" => points += 3, // d
            "B" => points += 0, // l
            "C" => points += 6, // w
            &_ => exit(1),
        }
    } else if player_pick == "P_B" {
        points += 2;
        match elf_pick {
            "A" => points += 6, // w
            "B" => points += 3, // d
            "C" => points += 0, // l
            &_ => exit(1),
        }
    } else if player_pick == "P_C" {
        points += 3;
        match elf_pick {
            "A" => points += 0, // l
            "B" => points += 6, // w
            "C" => points += 3, // d
            &_ => exit(1),
        }
    } else {
        exit(1);
    }
    return points;
}

fn decide_pick<'a>(elf_pick: &str, player_strategy: &str) -> &'a str {
    let player_pick: &'a str;

    if player_strategy == "X" {
        match elf_pick { // loose
            "A" => player_pick = "P_C",
            "B" => player_pick = "P_A",
            "C" => player_pick = "P_B",
            &_ => exit(1),
        }
    } else if player_strategy == "Y" {
        match elf_pick { // draw
            "A" => player_pick = "P_A",
            "B" => player_pick = "P_B",
            "C" => player_pick = "P_C",
            &_ => exit(1),
        }
    } else if player_strategy == "Z" {
        match elf_pick { // win
            "A" => player_pick = "P_B",
            "B" => player_pick = "P_C",
            "C" => player_pick = "P_A",
            &_ => exit(1),
        }
    } else {
        exit(1);
    }
    return player_pick;
}

fn main() {
    let strategies = lines_from_file("./day_2/day_2.in");
    let mut player_score: i32 = 0;

    for play in strategies {
        let picks: Vec<&str> = play.split_whitespace().collect();
        let elf_pick: &str = picks[0];
        let player_pick: &str = match picks[1] {
            "X" => "P_A",
            "Y" => "P_B",
            "Z" => "P_C",
            _ => picks[1],
        };
        player_score+= referee(elf_pick, player_pick);
    }

    println!("Player score pt1: {}", player_score);

    let strategies_pt2 = lines_from_file("./day_2/day_2.in");
    let mut player_score_pt2: i32 = 0;

    for play in strategies_pt2 {
        let picks: Vec<&str> = play.split_whitespace().collect();
        let elf_pick: &str = picks[0];
        let player_strategy: &str = picks[1];
        let player_pick: &str = decide_pick(elf_pick, player_strategy);

        player_score_pt2 += referee(elf_pick, player_pick)
    }

    println!("Player score pt2: {}", player_score_pt2)
}
