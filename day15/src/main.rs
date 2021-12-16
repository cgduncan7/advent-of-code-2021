use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn get_neighbors(position: (usize, usize), bounds: (usize, usize)) -> Vec<(usize, usize)> {
    let (x, y) = position;
    let mut neighbors: Vec<(usize, usize)> = Vec::new();

    if x > 0 {
        neighbors.push((x-1, y));
    }

    if x < bounds.0 - 1 {
        neighbors.push((x+1, y));
    }

    if y > 0 {
        neighbors.push((x, y-1));
    }

    if y < bounds.1 - 1 {
        neighbors.push((x, y+1));
    }

    neighbors
}

fn problem1(lines: &mut std::str::Lines) -> u32 {
    let mut points: HashMap<(usize, usize), u32> = HashMap::new();
    let bounds = (lines.clone().count(), lines.clone().count());
    for (y, line) in lines.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            points.insert((x, y), ch.to_digit(10).unwrap());
        }
    }

    let mut visited: Vec<(usize, usize)> = Vec::new();
    let mut leaves: Vec<((usize, usize), u32)> = Vec::new();
    leaves.push(((0, 0), 0));

    while !leaves.is_empty() {
        // leaf with lowest risk level
        let leaf = leaves.pop().unwrap();

        if leaf.0 == (bounds.0 - 1, bounds.1 - 1) {
            return leaf.1;
        }

        let neighbors = get_neighbors(leaf.0, bounds).iter()
            .filter(|&n| !visited.contains(n))
            .map(|&n| n)
            .collect::<Vec<(usize,usize)>>();

        for neighbor in neighbors {
            leaves.push((neighbor, leaf.1 + points.get(&neighbor).unwrap()));
            visited.push(neighbor);
        }

        // "max-heapify"; sort by risk level ascending and reverse
        leaves.sort_by_key(|leaf| leaf.1);
        leaves.reverse();
    }

    0
}

fn problem2(lines: &mut std::str::Lines) -> u32 {
    let mut points: HashMap<(usize, usize), u32> = HashMap::new();
    let bounds = (lines.clone().count(), lines.clone().count());
    for (y, line) in lines.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            for m in 0..25 {
                let row = (m / 5) as usize;
                let col = (m % 5) as usize;
                
                let addition = (m / 5) + (m % 5);
                let mut risk_level = ch.to_digit(10).unwrap() + addition;
                if risk_level > 9 {
                    risk_level = risk_level - 9;
                }
                
                points.insert((x + col * bounds.0, y + row * bounds.1), risk_level);
            }
        }
    }

    let bounds = (bounds.0 * 5, bounds.1 * 5);


    let mut visited: HashMap<(usize, usize), u32> = HashMap::new();
    let mut leaves: Vec<((usize, usize), u32, u32)> = Vec::new();
    leaves.push(((0, 0), 0, 0));

    while !leaves.is_empty() {
        // leaf with lowest risk level
        let leaf = leaves.pop().unwrap();

        if leaf.0 == (bounds.0 - 1, bounds.1 - 1) {
            return leaf.1;
        }

        let neighbors = get_neighbors(leaf.0, bounds).iter()
            .filter(|&n| {
                match visited.get(n) {
                    None => true,
                    Some(&val) => {
                        let distance: u32 = bounds.0 as u32 - n.0 as u32 - 1 + bounds.1 as u32 - n.1 as u32 - 1;
                        let cur_val = leaf.1 + points.get(&n).unwrap() + distance;
                        cur_val < val
                    }
                }
            })
            .map(|&n| n)
            .collect::<Vec<(usize,usize)>>();

        for neighbor in neighbors {
            let distance: u32 = bounds.0 as u32 - neighbor.0 as u32 - 1 + bounds.1 as u32 - neighbor.1 as u32 - 1;
            leaves.push((neighbor, leaf.1 + points.get(&neighbor).unwrap(), distance));
            visited.insert(neighbor, leaf.1 + points.get(&neighbor).unwrap() + distance);
        }

        // "max-heapify"; sort by risk level ascending and reverse
        leaves.sort_by(|a, b| {
            let (_, risk_a, dist_a) = a;
            let (_, risk_b, dist_b) = b;
            (risk_b + dist_b).cmp(&(risk_a + dist_a))
        });
    }

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
