use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn problem1(lines: &mut std::str::Lines) -> usize {
    let mut point_map: HashMap<(u32, u32), u32> = HashMap::new();

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }

        let line = line.unwrap();
        let points: (&str, &str) = line.split_once(" -> ").unwrap();
        let first_point: (u32, u32) = {
            let str_point = points.0.split_once(',').unwrap();
            (
                str_point.0.parse::<u32>().unwrap(),
                str_point.1.parse::<u32>().unwrap(),
            )
        };
        let second_point: (u32, u32) = {
            let str_point = points.1.split_once(',').unwrap();
            (
                str_point.0.parse::<u32>().unwrap(),
                str_point.1.parse::<u32>().unwrap(),
            )
        };

        if first_point.0 == second_point.0 {
            // vertical
            let x = first_point.0;
            let mut y: u32;
            let end_y: u32;
            if first_point.1 < second_point.1 {
                y = first_point.1;
                end_y = second_point.1;
            } else {
                y = second_point.1;
                end_y = first_point.1;
            }

            while y <= end_y {
                let pv = *(point_map.get(&(x, y)).unwrap_or(&0));
                point_map.insert((x, y), 1 + pv);
                y = y + 1;
            }
        } else if first_point.1 == second_point.1 {
            // horizontal
            let y = first_point.1;
            let mut x: u32;
            let end_x: u32;
            if first_point.0 < second_point.0 {
                x = first_point.0;
                end_x = second_point.0;
            } else {
                x = second_point.0;
                end_x = first_point.0;
            }

            while x <= end_x {
                let pv = *(point_map.get(&(x, y)).unwrap_or(&0));
                point_map.insert((x, y), 1 + pv);
                x = x + 1;
            }
        }
    }

    point_map.retain(|_, &mut v| v >= 2);
    point_map.len()
}

fn problem2(lines: &mut std::str::Lines) -> usize {
    let mut point_map: HashMap<(i32, i32), i32> = HashMap::new();

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }

        let line = line.unwrap();
        let points: (&str, &str) = line.split_once(" -> ").unwrap();
        let first_point: (i32, i32) = {
            let str_point = points.0.split_once(',').unwrap();
            (
                str_point.0.parse::<i32>().unwrap(),
                str_point.1.parse::<i32>().unwrap(),
            )
        };
        let second_point: (i32, i32) = {
            let str_point = points.1.split_once(',').unwrap();
            (
                str_point.0.parse::<i32>().unwrap(),
                str_point.1.parse::<i32>().unwrap(),
            )
        };

        if first_point.0 == second_point.0 {
            // vertical
            let x = first_point.0;
            let mut y: i32;
            let end_y: i32;
            if first_point.1 < second_point.1 {
                y = first_point.1;
                end_y = second_point.1;
            } else {
                y = second_point.1;
                end_y = first_point.1;
            }

            while y <= end_y {
                let pv = *(point_map.get(&(x, y)).unwrap_or(&0));
                point_map.insert((x, y), 1 + pv);
                y = y + 1;
            }
        } else if first_point.1 == second_point.1 {
            // horizontal
            let y = first_point.1;
            let mut x: i32;
            let end_x: i32;
            if first_point.0 < second_point.0 {
                x = first_point.0;
                end_x = second_point.0;
            } else {
                x = second_point.0;
                end_x = first_point.0;
            }

            while x <= end_x {
                let pv = *(point_map.get(&(x, y)).unwrap_or(&0));
                point_map.insert((x, y), 1 + pv);
                x = x + 1;
            }
        } else {
            // diagonal
            let mut x = first_point.0;
            let mut y = first_point.1;

            let mut x_step: i32 = 1;
            let mut y_step: i32 = 1;

            if x > second_point.0 {
                x_step = -1;
            }

            if y > second_point.1 {
                y_step = -1;
            }

            loop {
                let pv = *(point_map.get(&(x, y)).unwrap_or(&0));
                point_map.insert((x, y), 1 + pv);

                if x == second_point.0 {
                    break;
                }

                x = x + x_step;
                y = y + y_step;
            }
        }
    }

    point_map.retain(|_, &mut v| v >= 2);
    point_map.len()
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
