use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::cell::RefCell;

fn problem1(lines: &mut std::str::Lines) -> u32 {
    let mut polymer: String = String::new();
    let mut rules: HashMap<String, char> = HashMap::new();

    for (index, line) in lines.enumerate() {
        if index == 0 {
            polymer = String::from(line);
        }

        if index > 1 {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            rules.insert(String::from(lhs), rhs.chars().next().unwrap());
        }
    }

    let mut steps = 0;
    while steps < 10 {
        let mut temp_string = String::new();
        for i in 0..polymer.len()-1 {
            let mut result_string = String::with_capacity(3);
            let pair = polymer.get(i..i+2).unwrap();
            let rule_result = rules.get(pair).unwrap();
            
            result_string.push_str(pair);
            result_string.insert(1, *rule_result);

            // remove duplicate
            temp_string.pop();
            temp_string.push_str(&result_string);
        }

        polymer = temp_string;

        steps += 1;
    }

    let mut element_count: HashMap<char, u32> = HashMap::new();
    for ch in polymer.chars() {
        let previous_count;
        
        {
            previous_count = *element_count.get(&ch).unwrap_or(&0);
        }

        element_count.insert(ch, previous_count + 1);
    }

    let mut min: u32 = u32::MAX;
    let mut max: u32 = u32::MIN;
    for (_, &v) in element_count.iter() {
        if v > max {
            max = v;
        }

        if v < min {
            min = v;
        }
    }

    max - min
}

fn merge_maps(maps: Vec<HashMap<char, i128>>) -> HashMap<char, i128> {
    let mut merged = HashMap::new();

    for map in maps {
        for (k, v) in map {
            let existing = *(merged.get(&k).unwrap_or(&0));
            merged.insert(k, existing + v);
        }
    }

    merged
}

fn polymerization(polymer: String, rules: &HashMap<String, String>, steps_left: u8, memos: &RefCell<HashMap<(String, u8), HashMap<char, i128>>>) -> HashMap<char, i128> {
    if steps_left == 0 {
        let mut ret = HashMap::new();
        let (l, r) = polymer.split_at(1);
        if l.eq_ignore_ascii_case(r) {
            ret.insert(l.chars().next().unwrap(), 2);
        } else {
            ret.insert(l.chars().next().unwrap(), 1);
            ret.insert(r.chars().next().unwrap(), 1);
        }
        return ret;
    }

    // check memos
    if let Some(m) = memos.borrow().get(&(polymer.clone(), steps_left)) {
        return m.clone();
    }

    // AB -> ACB
    let next = rules.get(&polymer).unwrap().to_string();

    // AC
    let left_polymer = String::from(next.get(0..2).unwrap());

    // CB
    let right_polymer = String::from(next.get(1..).unwrap());

    let left = polymerization(left_polymer, rules, steps_left - 1, memos);
    let right = polymerization(right_polymer, rules, steps_left - 1, memos);

    // dedup
    let mut dedup_map = HashMap::new();
    dedup_map.insert(next.chars().skip(1).next().unwrap(), -1);

    let ret_val = merge_maps(vec![left, right, dedup_map]);
    memos.borrow_mut().insert((polymer, steps_left), ret_val.clone());

    ret_val
}

fn problem2(lines: &mut std::str::Lines) -> i128 {
    let mut polymer: String = String::new();
    let mut rules: HashMap<String, String> = HashMap::new();

    for (index, line) in lines.enumerate() {
        if index == 0 {
            polymer = String::from(line);
        }

        if index > 1 {
            let (lhs, rhs) = line.split_once(" -> ").unwrap();
            let mut result = String::from(lhs);
            result.insert(1, rhs.chars().next().unwrap());
            rules.insert(String::from(lhs), result);
        }
    }

    let memos: RefCell<HashMap<(String, u8), HashMap<char, i128>>> = RefCell::new(HashMap::new());
    let mut totals = Vec::new();
    for i in 0..polymer.len() - 1 {
        let pair = polymer.get(i..i+2).unwrap();
        if i+1 != polymer.len() - 1  {
            let mut dedup_map = HashMap::new();
            dedup_map.insert(polymer.get(i+1..i+2).unwrap().chars().next().unwrap(), -1);
            totals.push(dedup_map);
        }
        totals.push(polymerization(pair.to_owned(), &rules, 40, &memos));
    }

    let mut min: i128 = i128::MAX;
    let mut max: i128 = i128::MIN;
    for (_, &value) in merge_maps(totals).iter() {
        if value > max {
            max = value;
        }

        if value < min {
            min = value;
        }
    }

    max - min
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
