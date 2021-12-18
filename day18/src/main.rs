use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug)]
enum SnailfishNumberElement {
    SnailfishNumber(SnailfishNumber),
    Number(u8),
}

#[derive(Clone, Debug)]
struct SnailfishNumber {
    elements: Vec<SnailfishNumberElement>,
}

impl SnailfishNumberElement {
    fn to_string(&self) -> String {
        let mut st: String = String::new();

        match self {
            SnailfishNumberElement::Number(n) => {
                st.push_str(format!("{}", *n).as_str());
            },
            SnailfishNumberElement::SnailfishNumber(s) => {
                st.push('[');
                st.push_str(s.elements[0].to_string().as_str());
                st.push(',');
                st.push_str(s.elements[1].to_string().as_str());
                st.push(']');
            }
        };

        st
    }

    fn from_string(input: &str) -> Self {
        let mut sf_stack: Vec<SnailfishNumberElement> = Vec::new();
        for ch in input.chars() {
            match ch {
                '[' => {},
                ']' => {
                    let right = sf_stack.pop().unwrap();
                    let left = sf_stack.pop().unwrap();
                    let cur = SnailfishNumberElement::SnailfishNumber(SnailfishNumber { elements: vec![left, right] });
                    sf_stack.push(cur);
                },
                c => {
                    if c != ',' {
                        let digit = c.to_digit(10).unwrap() as u8;
                        sf_stack.push(SnailfishNumberElement::Number(digit));
                    }
                },
            }
        }
    
        sf_stack.pop().unwrap()
    }

    fn add(a: SnailfishNumberElement, b: SnailfishNumberElement) -> SnailfishNumberElement {
        SnailfishNumberElement::SnailfishNumber(SnailfishNumber {
            elements: vec![a, b]
        })
    }
    
    fn inc_right(s: SnailfishNumberElement, amt: u8) -> SnailfishNumberElement {
        if amt == 0 {
            return s;
        }
        match s {
            SnailfishNumberElement::SnailfishNumber(n) => {
                return SnailfishNumberElement::SnailfishNumber(SnailfishNumber {
                    elements: vec![
                        SnailfishNumberElement::inc_right(n.elements[0].clone(), amt),
                        n.elements[1].clone(),
                    ],
                });
            }
            SnailfishNumberElement::Number(n) => {
                return SnailfishNumberElement::Number(n + amt);
            }
        }
    }
    
    fn inc_left(s: SnailfishNumberElement, amt: u8) -> SnailfishNumberElement {
        if amt == 0 {
            return s;
        }
        match s {
            SnailfishNumberElement::SnailfishNumber(n) => {
                return SnailfishNumberElement::SnailfishNumber(SnailfishNumber {
                    elements: vec![
                        n.elements[0].clone(),
                        SnailfishNumberElement::inc_left(n.elements[1].clone(), amt),
                    ],
                });
            }
            SnailfishNumberElement::Number(n) => {
                return SnailfishNumberElement::Number(n + amt);
            }
        }
    }
    
    fn explode(sfne: SnailfishNumberElement, depth: u8, has_explode: bool) -> (SnailfishNumberElement, u8, u8, bool) {
        match &sfne {
            SnailfishNumberElement::SnailfishNumber(sfn) => {
                if depth == 4 {
                    // explode
                    let left_val = match &sfn.elements[0] {
                        SnailfishNumberElement::Number(n) => n,
                        s => panic!("Unexpected SnailfishNumber {:?}", s.clone().to_string())
                    };
                    let right_val = match &sfn.elements[1] {
                        SnailfishNumberElement::Number(n) => n,
                        s => panic!("Unexpected SnailfishNumber {:?}", s.clone().to_string())
                    };
                    
                    let to_return = (SnailfishNumberElement::Number(0), *left_val, *right_val, true);
                    return to_return;
                }
    
                let (lv, lla, lra, lr) = SnailfishNumberElement::explode(sfn.elements[0].clone(), depth + 1, has_explode);
    
                if !lr {
                    let (rv, rla, rra, rr) = SnailfishNumberElement::explode(sfn.elements[1].clone(), depth + 1, has_explode);
                    
                    if rr {
                        let to_return = (SnailfishNumberElement::SnailfishNumber(
                            SnailfishNumber { elements: vec![SnailfishNumberElement::inc_left(lv, rla), rv] }), 0, rra, rr);
                        return to_return;
                    } else {
                        let to_return = (SnailfishNumberElement::SnailfishNumber(
                            SnailfishNumber { elements: vec![lv, rv] }), 0, 0, false);
                        return to_return;
                    }
                    
                } else {
                    let to_return = (SnailfishNumberElement::SnailfishNumber(
                        SnailfishNumber{ elements: vec![lv, SnailfishNumberElement::inc_right(sfn.elements[1].clone(), lra)] }), lla, 0, lr);
                    return to_return;
                }
    
    
            },
            _ => {
                return (sfne, 0, 0, false);
            }
        };
    }

    fn split(sfne: SnailfishNumberElement, depth: u8, has_split: bool) -> (SnailfishNumberElement, u8, u8, bool) {
        match &sfne {
            SnailfishNumberElement::SnailfishNumber(sfn) => {    
                let (lv, _, _, lr) = SnailfishNumberElement::split(sfn.elements[0].clone(), depth + 1, has_split);
                let (rv, _, _, rr) = SnailfishNumberElement::split(sfn.elements[1].clone(), depth + 1, lr || has_split);
                return (SnailfishNumberElement::SnailfishNumber(SnailfishNumber {
                    elements: vec![lv, rv]
                }), 0, 0, lr || rr);
            },
            SnailfishNumberElement::Number(n) => {
                if n >= &10 && !has_split {
                    // split
                    let left_val = SnailfishNumberElement::Number(n / 2);
                    let right_val = SnailfishNumberElement::Number(
                        if n % 2 == 0 {
                            n / 2
                        } else {
                            1 + (n / 2)
                        }
                    );
    
                    return (SnailfishNumberElement::SnailfishNumber(SnailfishNumber { elements: vec![left_val, right_val] }), 0, 0, true);
                }
                return (sfne, 0, 0, false);
            }
        };
    
    }

    fn calculate_magnitude(&self) -> u64 {
        match self {
            SnailfishNumberElement::Number(n) => *n as u64,
            SnailfishNumberElement::SnailfishNumber(s) => {
                3 * s.elements[0].calculate_magnitude() +
                2 * s.elements[1].calculate_magnitude()
            }
        }
    }
}

fn problem1(lines: &mut std::str::Lines) -> u64 {
    let init = SnailfishNumberElement::from_string(lines.next().unwrap());
    
    {
        let mut s = init;
        for line in lines {
            s = SnailfishNumberElement::add(s.clone(), SnailfishNumberElement::from_string(line));
            loop {
                let (new_s, _, _, exploded) = SnailfishNumberElement::explode(s.clone(), 0, false);

                if !exploded {
                    let (new_s, _, _, split) = SnailfishNumberElement::split(s.clone(), 0, false);
                    s = new_s.clone();
                    
                    if !split {
                        break;
                    }
                } else {
                    s = new_s.clone();
                }
            }
        }

        s.calculate_magnitude()
    }
}

fn problem2(lines: &mut std::str::Lines) -> u64 {
    let mut max = (0, Vec::new());
    for (index, line) in lines.clone().enumerate() {
        for (inner_index, inner_line) in lines.clone().enumerate() {
            if index == inner_index {
                continue;
            }
            let mut s = SnailfishNumberElement::from_string(line);
            s = SnailfishNumberElement::add(s.clone(), SnailfishNumberElement::from_string(inner_line));
            loop {
                let (new_s, _, _, exploded) = SnailfishNumberElement::explode(s.clone(), 0, false);

                if !exploded {
                    let (new_s, _, _, split) = SnailfishNumberElement::split(s.clone(), 0, false);
                    s = new_s.clone();
                    
                    if !split {
                        break;
                    }
                } else {
                    s = new_s.clone();
                }
            }
    
            let this_mag = s.calculate_magnitude();

            if this_mag > max.0 {
                max.0 = this_mag;
                max.1 = vec![index, inner_index];
            }
        }
    }
    max.0
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