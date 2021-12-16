use std::fs::File;
use std::io::prelude::*;

fn problem1(lines: std::str::Lines) -> u32 {
    // (horizontal, vertical)
    let mut position: (u32, u32) = (0, 0);

    lines.for_each(|line| {
        let words: Vec<&str> = line.split(' ').collect();
        let num: u32 = match words[1].parse() {
            Err(_) => panic!("Failed to parse number"),
            Ok(v) => v,
        };
        match words[0] {
            "forward" => position = (position.0 + num, position.1),
            "up" => position = (position.0, position.1 - num),
            "down" => position = (position.0, position.1 + num),
            _ => (),
        }
    });

    position.0 * position.1
}

fn problem2(lines: std::str::Lines) -> u32 {
    // (horizontal, vertical, aim)
    let mut position: (u32, u32, u32) = (0, 0, 0);

    lines.for_each(|line| {
        let words: Vec<&str> = line.split(' ').collect();
        let num: u32 = match words[1].parse() {
            Err(_) => panic!("Failed to parse number"),
            Ok(v) => v,
        };
        match words[0] {
            "forward" => {
                position = (
                    position.0 + num,
                    position.1 + (position.2 * num),
                    position.2,
                )
            }
            "up" => position = (position.0, position.1, position.2 - num),
            "down" => position = (position.0, position.1, position.2 + num),
            _ => (),
        }
    });

    position.0 * position.1
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

    println!("Problem1: {}", problem1(lines.clone()));
    println!("Problem1: {}", problem2(lines.clone()));
}
