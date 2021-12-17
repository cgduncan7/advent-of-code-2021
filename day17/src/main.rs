use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn problem1(lines: &mut std::str::Lines) -> i32 {
    let input = lines.next().unwrap();

    let (_, target_area) = input.split_once(": ").unwrap();
    let (_, y_range) = target_area.split_once(", ").unwrap();
    let (min_y_str, _) = y_range.strip_prefix("y=").unwrap().split_once("..").unwrap();

    let min_y = min_y_str.parse::<i32>().unwrap();

    let y = min_y.abs() - 1;

    y * (y+1) / 2
}

fn problem2(lines: &mut std::str::Lines) -> usize {
    let input = lines.next().unwrap();

    let (_, target_area) = input.split_once(": ").unwrap();
    let (x_range, y_range) = target_area.split_once(", ").unwrap();
    let (min_x_str, max_x_str) = x_range.strip_prefix("x=").unwrap().split_once("..").unwrap();
    let (min_y_str, max_y_str) = y_range.strip_prefix("y=").unwrap().split_once("..").unwrap();

    let min_x = min_x_str.parse::<i32>().unwrap();
    let max_x = max_x_str.parse::<i32>().unwrap();

    let min_y = min_y_str.parse::<i32>().unwrap();
    let max_y = max_y_str.parse::<i32>().unwrap();

    // calculate possible x vel
    let mut possible_x: HashMap<i32, Vec<u32>> = HashMap::new();
    let max_possible_x = max_x;
    let min_possible_x = ((-1. + (1. + 8. * min_x as f32).sqrt()) / 2.).ceil() as i32;

    for x in min_possible_x..max_possible_x+1 {
        let mut distance = 0;
        let mut xi = x.clone();
        let mut steps = 0;
        while distance < max_x {
            steps += 1;
            distance += xi;
            if xi > 0 {
                xi -=1;
            }
            if distance >= min_x && distance <= max_x {
                let mut v = possible_x.get(&x).unwrap_or(&Vec::new()).to_owned();
                if xi <= 0 {
                    v.push(u32::MAX);
                }
                v.push(steps);
                v.sort();
                possible_x.insert(x, v);
            }

            if xi == 0 {
                break
            }
        }
    }
    
    // calculate possible y vel
    let mut possible_y: HashMap<i32, Vec<u32>> = HashMap::new();
    let y = min_y.abs() - 1;
    let max_possible_y = y * (y+1) / 2;
    let min_possible_y = min_y;

    for y in min_possible_y..max_possible_y+1 {
        let mut distance = 0;
        let mut yi = y.clone();
        let mut steps = 0;
        while distance > min_y {
            steps += 1;
            distance += yi;
            if distance >= min_y && distance <= max_y {
                let mut v = possible_y.get(&y).unwrap_or(&Vec::new()).to_owned();
                v.push(steps);
                v.sort();
                possible_y.insert(y, v);
            }
            yi -=1;
        }
    }

    let mut possible_vels: Vec<(i32, i32)> = Vec::new();

    for (&val_x, steps_x) in possible_x.iter() {
        for (&val_y, steps_y) in possible_y.iter() {
            for sy in steps_y.clone() {
                if steps_x.contains(&sy) {
                    possible_vels.push((val_x, val_y));
                }

                // if val_x stops in target (e.g. x_vel reaches 0 inside of target),
                // any val_y which needs steps >= val_x will work for this val_x
                if steps_x.contains(&u32::MAX) {
                    if sy as i32 >= val_x {
                        possible_vels.push((val_x, val_y));
                    }
                }
            }
        }
    }

    possible_vels.sort_by_key(|v| v.0);
    possible_vels.dedup();

    possible_vels.len()
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
