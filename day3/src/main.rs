use std::fs::File;
use std::io::prelude::*;

fn problem1(lines: &std::str::Lines) -> u32 {
    let mut sums: Vec<u32> = Vec::new();
    for line in lines.clone() {
        if sums.len() == 0 {
            sums = vec![0; line.chars().count()];
        }

        let mut index = 0;
        for c in line.chars() {
            if c == '1' {
                sums[index] = sums[index] + 1;
            }
            index = index + 1;
        }
    }

    let mut most_common_bits = String::new();
    let mut least_common_bits = String::new();
    for sum in sums {
        if sum >= (lines.clone().count() as u32 / 2) {
            most_common_bits.push('1');
            least_common_bits.push('0');
        } else {
            most_common_bits.push('0');
            least_common_bits.push('1');
        }
    }

    let gamma = match u32::from_str_radix(most_common_bits.as_str(), 2) {
        Err(_) => panic!("Failed to parse gamma"),
        Ok(v) => v,
    };
    let epsilon = match u32::from_str_radix(least_common_bits.as_str(), 2) {
        Err(_) => panic!("Failed to parse epsilon"),
        Ok(v) => v,
    };

    gamma * epsilon
}

fn problem2(lines: &std::str::Lines) -> u32 {
    let mut oxygen_possibilities: Vec<&str> = lines.clone().map(|l| l).collect();
    let mut co2_possibilities: Vec<&str> = oxygen_possibilities.clone();

    let line_length = match oxygen_possibilities.get(0) {
        None => panic!("No lines provided"),
        Some(l) => l.len(),
    };

    for char_index in 0..line_length {
        let mut o_sum = 0;
        let mut c_sum = 0;

        if oxygen_possibilities.len() > 1 {
            for line in &oxygen_possibilities {
                let c: char = match line.chars().nth(char_index) {
                    None => panic!("Character not found"),
                    Some(v) => v,
                };

                if c == '1' {
                    o_sum = o_sum + 1;
                }
            }

            let most_common_value = if o_sum as f32 >= (oxygen_possibilities.len() as f32 / 2.0) {
                '1'
            } else {
                '0'
            };

            oxygen_possibilities = oxygen_possibilities
                .into_iter()
                .filter(|p| match p.chars().nth(char_index) {
                    None => false,
                    Some(v) => v == most_common_value,
                })
                .collect();
        }

        if co2_possibilities.len() > 1 {
            for line in &co2_possibilities {
                let c: char = match line.chars().nth(char_index) {
                    None => panic!("Character not found"),
                    Some(v) => v,
                };

                if c == '1' {
                    c_sum = c_sum + 1;
                }
            }

            let most_common_value = if c_sum as f32 >= (co2_possibilities.len() as f32 / 2.0) {
                '1'
            } else {
                '0'
            };

            co2_possibilities = co2_possibilities
                .into_iter()
                .filter(|p| match p.chars().nth(char_index) {
                    None => false,
                    Some(v) => v != most_common_value,
                })
                .collect();
        }
    }

    let o2_rating: u32 = u32::from_str_radix(oxygen_possibilities.first().unwrap(), 2).unwrap();
    let co2_rating: u32 = u32::from_str_radix(co2_possibilities.first().unwrap(), 2).unwrap();

    o2_rating * co2_rating
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
    println!("Problem1: {}", problem1(&lines));
    println!("Problem2: {}", problem2(&lines));
}
