use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

struct Lanternfish {
    days_till_bebe: u8,
}

impl Lanternfish {
    pub fn init(days_till_bebe: u8) -> Lanternfish {
        Lanternfish {
            days_till_bebe: days_till_bebe,
        }
    }

    pub fn new() -> Lanternfish {
        Lanternfish { days_till_bebe: 8 }
    }

    pub fn age(self: &mut Lanternfish) -> Option<Lanternfish> {
        if self.days_till_bebe == 0 {
            // make a bebe
            self.days_till_bebe = 6;
            Some(Lanternfish::new())
        } else {
            self.days_till_bebe = self.days_till_bebe - 1;
            None
        }
    }
}

fn problem1(lines: &mut std::str::Lines) -> usize {
    let mut fishes: Vec<Lanternfish> = lines
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| Lanternfish::init(v.parse::<u8>().unwrap()))
        .collect();

    for _ in 0..80 {
        let mut bebe_fishes: Vec<Lanternfish> = Vec::new();
        for fish in &mut fishes {
            if let Some(bebe_fish) = fish.age() {
                bebe_fishes.push(bebe_fish);
            }
        }

        fishes.append(&mut bebe_fishes);
    }

    fishes.len()
}

fn calculate_fish_footprint_memo(
    memoized_fishes: &mut HashMap<(u128, u128), u128>,
    day_born: u128,
    days_till_bebe: u128,
) -> u128 {
    if let Some(v) = memoized_fishes.get(&(day_born, days_till_bebe)) {
        return *v;
    }

    if 80 < days_till_bebe + days_till_bebe {
        memoized_fishes.insert((day_born, days_till_bebe), 1);
        return 1;
    }

    let mut footprint: u128 = 1;

    let dtb = 7;
    let mut day = day_born + days_till_bebe;
    while day <= 256 {
        footprint = footprint + calculate_fish_footprint_memo(memoized_fishes, day, 9);
        day = day + dtb;
    }

    memoized_fishes.insert((day_born, days_till_bebe), footprint);
    footprint
}

fn problem2(lines: &mut std::str::Lines) -> u128 {
    let fish_dtb: Vec<u128> = lines
        .next()
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|v| v.parse::<u128>().unwrap())
        .collect();

    let mut memo_fish: HashMap<(u128, u128), u128> = HashMap::new();
    let mut total_fish: u128 = 0;
    for dtb in fish_dtb {
        total_fish = total_fish + calculate_fish_footprint_memo(&mut memo_fish, 0, dtb + 1);
    }

    total_fish
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
