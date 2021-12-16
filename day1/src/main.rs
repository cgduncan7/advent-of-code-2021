use std::fs::File;
use std::io::prelude::*;

fn problem1(contents: &mut str) -> u32 {
    let mut previous_depth: i32 = -1;
    contents.lines().fold(0, |accumulator, line| {
        let depth: i32 = match line.parse() {
            Err(_) => panic!("Failed to parse {}", line),
            Ok(v) => v,
        };

        if previous_depth != -1 && depth > previous_depth {
            previous_depth = depth;
            accumulator + 1
        } else {
            previous_depth = depth;
            accumulator
        }
    })
}

fn problem2(contents: &mut str) -> u32 {
    let mut depths: Vec<u32> = Vec::new();
    contents.lines().for_each(|x| match x.parse() {
        Err(_) => panic!("Failed to parse"),
        Ok(v) => depths.push(v),
    });

    let mut windows: Vec<(u32, u32, u32)> = Vec::new();
    for x in 0..depths.len() - 1 {
        let window = (
            match depths.get(x) {
                None => 0,
                Some(x) => *x,
            },
            match depths.get(x + 1) {
                None => 0,
                Some(x) => *x,
            },
            match depths.get(x + 2) {
                None => 0,
                Some(x) => *x,
            },
        );

        windows.push(window);
    }

    let mut increases: u32 = 0;

    let mut previous_window_sum: u32 = 0;
    for window in windows {
        let window_sum = window.0 + window.1 + window.2;
        if previous_window_sum != 0 {
            if window_sum > previous_window_sum {
                increases = increases + 1;
            }
        }

        previous_window_sum = window_sum;
    }

    increases
}

fn main() {
    let mut file = match File::open("./data/input1.txt") {
        Err(_) => panic!("Failed to open file"),
        Ok(f) => f,
    };
    let mut contents = String::new();
    if file.read_to_string(&mut contents).is_err() {
        panic!("Failed to read file");
    }

    println!("Problem1: {}", problem1(&mut contents));
    println!("Problem2: {}", problem2(&mut contents));
}
