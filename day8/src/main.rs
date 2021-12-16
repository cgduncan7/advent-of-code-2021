use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn problem1(lines: &mut std::str::Lines) -> u32 {
    let mut count: u32 = 0;

    for line in lines {
        count = count
            + line
                .split_once(" | ")
                .unwrap()
                .1
                .split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .fold(0, |acc, cur| {
                    let mut to_add = 0;
                    if cur.len() == 2 || cur.len() == 3 || cur.len() == 4 || cur.len() == 7 {
                        to_add = 1;
                    }
                    acc + to_add
                });
    }

    count
}

fn xor(op1: String, op2: String) -> String {
    let mut combined = op1.to_owned();
    combined.push_str(op2.as_str());

    let mut deduped_str: String = String::new();
    for ch in combined.chars() {
        if !deduped_str.contains(ch) {
            deduped_str.push(ch);
        } else {
            deduped_str.remove(deduped_str.find(ch).unwrap());
        }
    }

    deduped_str
}

fn heuristic_fn(input: &String, digit_to_decoded_str: &HashMap<u32, String>) -> u32 {
    if input.len() == 2 {
        return 1;
    }

    if input.len() == 3 {
        return 7;
    }

    if input.len() == 4 {
        return 4;
    }

    if input.len() == 7 {
        return 8;
    }

    if input.len() == 5 {
        let s1 = digit_to_decoded_str.get(&1).unwrap();
        let x = xor(input.to_string(), s1.to_string());
        if x.len() == 3 {
            return 3;
        }

        let s4 = digit_to_decoded_str.get(&4).unwrap();
        let x = xor(x, s4.to_string());
        if x.len() == 5 {
            return 2;
        }

        return 5;
    }

    if input.len() == 6 {
        let s5 = digit_to_decoded_str.get(&5).unwrap();
        let x = xor(input.to_string(), s5.to_string());
        if x.len() == 3 {
            return 0;
        }

        let s1 = digit_to_decoded_str.get(&1).unwrap();
        let x = xor(x, s1.to_string());
        if x.len() == 3 {
            return 6;
        }

        return 9;
    }

    panic!("Shouldn't be here!");
}

fn problem2(lines: &mut std::str::Lines) -> u32 {
    let mut total: u32 = 0;

    for line in lines {
        let mut decoded_str_to_digit: HashMap<String, u32> = HashMap::new();
        let mut digit_to_decoded_str: HashMap<u32, String> = HashMap::new();

        let (lhs, rhs) = line.split_once(" | ").unwrap();
        let encoded_digits = lhs.split_whitespace().collect::<Vec<&str>>();

        let mut encoded_digits: Vec<String> = encoded_digits
            .iter()
            .map(|s| {
                let mut sorted_deduped_str: String = String::new();
                let mut vc = s.chars().collect::<Vec<char>>();
                vc.sort();

                for ch in vc {
                    sorted_deduped_str.push(ch);
                }

                sorted_deduped_str
            })
            .collect();

        // sort by encoding length
        encoded_digits.sort_by(|a, b| a.len().cmp(&b.len()));

        for ed in encoded_digits {
            let result = heuristic_fn(&ed, &digit_to_decoded_str);
            decoded_str_to_digit.insert(ed.to_string(), result);
            digit_to_decoded_str.insert(result, ed.to_string());
        }

        let display_digits = rhs.split_whitespace().collect::<Vec<&str>>();
        let mut display_digits: Vec<String> = display_digits
            .iter()
            .map(|s| {
                let mut sorted_deduped_str: String = String::new();
                let mut vc = s.chars().collect::<Vec<char>>();
                vc.sort();

                for ch in vc {
                    sorted_deduped_str.push(ch);
                }

                sorted_deduped_str
            })
            .collect();

        display_digits.reverse();

        let mut decoded_display: u32 = 0;
        for (i, d) in display_digits.iter().enumerate() {
            let val = *(decoded_str_to_digit.get(d).unwrap());
            decoded_display = decoded_display + (val * 10u32.pow(i as u32));
        }

        total = total + decoded_display;
    }

    total
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
