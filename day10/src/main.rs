use std::fs::File;
use std::io::prelude::*;

const OPEN_CHARS: [char; 4] = ['(', '[', '{', '<'];
const VALID_CHUNKS: [&str; 4] = ["()", "[]", "{}", "<>"];

fn problem1(lines: &mut std::str::Lines) -> u32 {
    let mut total_score = 0;

    let mut parsing_stack: Vec<char> = Vec::new();
    for line in lines {
        for ch in line.chars() {
            if OPEN_CHARS.contains(&ch) {
                parsing_stack.push(ch);
            } else {
                let chunk_start = parsing_stack.pop().unwrap();
                let mut chunk = String::new();
                chunk.push(chunk_start);
                chunk.push(ch);

                if !VALID_CHUNKS.contains(&chunk.as_str()) {
                    total_score = total_score + match ch {
                        ')' => 3,
                        ']' => 57,
                        '}' => 1197,
                        '>' => 25137,
                        _ => 0,
                    };

                    break;
                }
            }
        }
    }

    total_score
}

fn problem2(lines: &mut std::str::Lines) -> u128 {
    let mut scores: Vec<u128> = Vec::new();
    let mut parsing_stack: Vec<char> = Vec::new();
    for line in lines {
        let mut is_corrupted = false;
        for ch in line.chars() {
            if OPEN_CHARS.contains(&ch) {
                parsing_stack.push(ch);
            } else {
                let chunk_start = parsing_stack.pop().unwrap();
                let mut chunk = String::new();
                chunk.push(chunk_start);
                chunk.push(ch);

                if !VALID_CHUNKS.contains(&chunk.as_str()) {
                    // corrupted line
                    is_corrupted = true;
                    break;
                }
            }
        }

        if is_corrupted {
            parsing_stack.clear();
            continue;
        }

        let mut line_score: u128 = 0;
        while !parsing_stack.is_empty() {
            let incomplete_chunk = parsing_stack.pop().unwrap();
            line_score = line_score * 5 + match incomplete_chunk {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
        }

        scores.push(line_score);
    }

    scores.sort();
    *scores.get(scores.len() / 2).unwrap()
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
