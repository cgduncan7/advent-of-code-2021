use std::fs::File;
use std::io::prelude::*;

fn problem1(lines: &mut std::str::Lines) -> u128 {
    let mut crab_positions: Vec<u128> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u128>().unwrap())
        .collect();

    crab_positions.sort();

    let move_to: u128 = crab_positions[crab_positions.len() / 2];

    crab_positions.iter().fold(0, |acc, &val| {
        if val < move_to {
            return acc + move_to - val;
        }
        acc + val - move_to
    })
}

fn problem2(lines: &mut std::str::Lines) -> u128 {
    let mut crab_positions: Vec<u128> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<u128>().unwrap())
        .collect();

    crab_positions.sort();

    let mut lowest_fuel = u128::MAX;
    for move_to in 0..crab_positions[crab_positions.len() - 1] {
        let potential = crab_positions.iter().fold(0, |acc, val| {
            let diff;
            if *val < move_to {
                diff = move_to - val;
            } else {
                diff = val - move_to;
            }
            acc + (diff * (diff + 1)) / 2
        });

        if potential < lowest_fuel {
            lowest_fuel = potential;
        }
    }

    lowest_fuel
}

fn main() {
    let mut file = match File::open("./data/input.txt") {
        Err(_) => panic!("Failed to open file"),
        Ok(f) => f,
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        panic!("Failed to read file");
    }

    let lines = contents.lines();
    println!("Problem1: {}", problem1(&mut lines.clone()));
    println!("Problem2: {}", problem2(&mut lines.clone()));
}
