use std::fs::File;
use std::io::prelude::*;

fn problem1(lines: &mut std::str::Lines) -> usize {
    let mut points: Vec<(u32, u32)> = Vec::new();

    for line in lines {
        if line.len() > 0 {
            if !line.starts_with("fold") {
                // plot points
                let (a, b) = line.split_once(',').unwrap();
                let point = (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());
                points.push(point);
            } else {
                // start folding
                let (lhs, rhs) = {
                    let x = line.replace("fold along ", "");
                    let y = x.split_once("=").unwrap();
                    (y.0.to_owned(), y.1.parse::<u32>().unwrap())
                };

                if lhs.as_str().eq_ignore_ascii_case("x") {
                    let mut new_points = points.iter()
                        .filter(|&point| point.0 > rhs)
                        .map(|point| {
                            (rhs - (point.0 - rhs), point.1)
                        })
                        .collect::<Vec<(u32, u32)>>();

                    let mut old_points = points.iter()
                        .filter(|p| p.0 < rhs)
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(u32,u32)>>();
                        
                    old_points.append(&mut new_points);
                    points.clear();
                    points.append(&mut old_points);
                } else {
                    let mut new_points = points.iter()
                        .filter(|&point| point.1 > rhs)
                        .map(|point| {
                            (point.0, rhs - (point.1 - rhs))
                        })
                        .collect::<Vec<(u32, u32)>>();

                    let mut old_points = points.iter()
                        .filter(|p| p.1 < rhs)
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(u32,u32)>>();
                        
                    old_points.append(&mut new_points);
                    points.clear();
                    points.append(&mut old_points);
                }
                break;
            }
        }
    }

    points.sort_by_key(|v| v.1);
    points.sort_by_key(|v| v.0);
    points.dedup();

    points.len()
}

fn problem2(lines: &mut std::str::Lines) -> u32 {
    let mut points: Vec<(u32, u32)> = Vec::new();

    for line in lines {
        if line.len() > 0 {
            if !line.starts_with("fold") {
                // plot points
                let (a, b) = line.split_once(',').unwrap();
                let point = (a.parse::<u32>().unwrap(), b.parse::<u32>().unwrap());
                points.push(point);
            } else {
                // start folding
                let (lhs, rhs) = {
                    let x = line.replace("fold along ", "");
                    let y = x.split_once("=").unwrap();
                    (y.0.to_owned(), y.1.parse::<u32>().unwrap())
                };

                if lhs.as_str().eq_ignore_ascii_case("x") {
                    let mut new_points = points.iter()
                        .filter(|&point| point.0 > rhs)
                        .map(|point| {
                            (rhs - (point.0 - rhs), point.1)
                        })
                        .collect::<Vec<(u32, u32)>>();

                    let mut old_points = points.iter()
                        .filter(|p| p.0 < rhs)
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(u32,u32)>>();
                        
                    old_points.append(&mut new_points);
                    points.clear();
                    points.append(&mut old_points);
                } else {
                    let mut new_points = points.iter()
                        .filter(|&point| point.1 > rhs)
                        .map(|point| {
                            (point.0, rhs - (point.1 - rhs))
                        })
                        .collect::<Vec<(u32, u32)>>();

                    let mut old_points = points.iter()
                        .filter(|p| p.1 < rhs)
                        .map(|p| (p.0, p.1))
                        .collect::<Vec<(u32,u32)>>();
                        
                    old_points.append(&mut new_points);
                    points.clear();
                    points.append(&mut old_points);
                }
            }
        }
    }

    points.sort_by_key(|v| v.1);
    let max_y = points[points.len() - 1].1;
    points.sort_by_key(|v| v.0);
    let max_x = points[points.len() - 1].0;
    points.dedup();

    println!();
    for y in 0..max_y+1 {
        for x in 0..max_x+1 {
            let point = (x, y);
            if points.contains(&point) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
    0
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
