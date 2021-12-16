use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

enum NodeType {
    Start,
    End,
    BigCave,
    SmallCave,
}

struct Node {
    node_type: NodeType,
    value: String,
    connections: RefCell<Vec<Rc<Node>>>,
}

impl Node {
    pub fn new(node_type: NodeType, value: String) -> Self {
        Node {
            node_type,
            value,
            connections: RefCell::new(Vec::new()),
        }
    }

    fn filter_fn(&self, next: &Rc<Node>, current: Vec<String>, problem2: bool) -> bool {
        if !problem2 {
            next.value.as_str().chars().all(|x| x.is_ascii_uppercase()) ||
                !current.contains(&next.value)
        } else {
            if next.value.as_str().chars().all(|x| x.is_ascii_uppercase()) {
                return true;
            }

            if next.value.as_str() == "start" {
                return false;
            }

            let mut lowercase_strings = current.iter()
                .filter(|&s| s.as_str().chars().all(|c| c.is_ascii_lowercase()))
                .collect::<Vec<&String>>();
            lowercase_strings.sort();
            
            let mut occurrences = 1;
            let mut prev_str = String::from("");
            let mut two_small_caves = false;
            for c in lowercase_strings.iter() {
                if c.eq_ignore_ascii_case(prev_str.as_str()) {
                    occurrences += 1;
                } else {
                    occurrences = 1;
                    prev_str = c.to_string();
                }

                if occurrences == 2 {
                    two_small_caves = true;
                }
            }

            !(two_small_caves && current.contains(&next.value))
        }
    }

    fn traverse_children(&self, current: Vec<String>, problem2: bool) -> Vec<Vec<String>> {
        self.connections.borrow().iter()
            .filter(|&c| self.filter_fn(c, current.clone(), problem2))
            .map(|c| {
                let child_connection_traversals = c.traverse(current.clone(), false, problem2);

                let mut ret_vec: Vec<Vec<String>> = Vec::new();
                child_connection_traversals.iter().for_each(|cct| {
                    ret_vec.push(cct.clone());
                });

                ret_vec
            }).collect::<Vec<Vec<Vec<String>>>>().concat()
    }

    pub fn traverse(&self, previous: Vec<String>, is_begin: bool, problem2: bool) -> Vec<Vec<String>> {
        let mut current = previous.clone();
        current.push(self.value.clone());
        
        match self.node_type {
            NodeType::End => {
                vec![current.clone()]
            },

            NodeType::Start => {
                if !is_begin {
                    return vec![current.clone()];
                }

                self.traverse_children(current, problem2)
            },

            _ => {        
                self.traverse_children(current, problem2)
            },
        }
    }
}

fn make_connection(a: Rc<Node>, b: Rc<Node>) {
    {
        let mut connections = a.connections.borrow_mut();
        connections.push(Rc::clone(&b));
    }

    if !a.value.eq_ignore_ascii_case("start") || !b.value.eq_ignore_ascii_case("end") {
        let mut connections = b.connections.borrow_mut();
        connections.push(Rc::clone(&a));
    }
}

fn problem1(lines: &mut std::str::Lines) -> usize {
    let mut node_map: HashMap<String, Rc<Node>> = HashMap::new();

    for line in lines {
        let (a, b) = line.split_once('-').unwrap();

        let node_a = match node_map.get(a) {
            None => {
                let node_type = match a {
                    "start" => NodeType::Start,
                    "end" => NodeType::End,
                    _ => {
                        if a.chars().all(|x| x.is_ascii_uppercase()) {
                            NodeType::BigCave
                        } else {
                            NodeType::SmallCave
                        }
                    }
                };
                let node = Rc::new(Node::new(node_type, String::from(a)));
                node_map.insert(a.to_owned(), Rc::clone(&node));
                node
            },
            Some(n) => Rc::clone(&n),
        };

        let node_b = match node_map.get(b) {
            None => {
                let node_type = match b {
                    "start" => NodeType::Start,
                    "end" => NodeType::End,
                    _ => {
                        if a.chars().all(|x| x.is_ascii_uppercase()) {
                            NodeType::BigCave
                        } else {
                            NodeType::SmallCave
                        }
                    }
                };
                let node = Rc::new(Node::new(node_type, String::from(b)));
                node_map.insert(b.to_owned(), Rc::clone(&node));
                node
            },
            Some(n) => Rc::clone(&n),
        };

        make_connection(node_a, node_b);
    }
    
    let start = node_map.get("start").unwrap();

    start.traverse(vec![], true, false).len()
}

fn problem2(lines: &mut std::str::Lines) -> usize {
    let mut node_map: HashMap<String, Rc<Node>> = HashMap::new();

    for line in lines {
        let (a, b) = line.split_once('-').unwrap();

        let node_a = match node_map.get(a) {
            None => {
                let node_type = match a {
                    "start" => NodeType::Start,
                    "end" => NodeType::End,
                    _ => {
                        if a.chars().all(|x| x.is_ascii_uppercase()) {
                            NodeType::BigCave
                        } else {
                            NodeType::SmallCave
                        }
                    }
                };
                let node = Rc::new(Node::new(node_type, String::from(a)));
                node_map.insert(a.to_owned(), Rc::clone(&node));
                node
            },
            Some(n) => Rc::clone(&n),
        };

        let node_b = match node_map.get(b) {
            None => {
                let node_type = match b {
                    "start" => NodeType::Start,
                    "end" => NodeType::End,
                    _ => {
                        if a.chars().all(|x| x.is_ascii_uppercase()) {
                            NodeType::BigCave
                        } else {
                            NodeType::SmallCave
                        }
                    }
                };
                let node = Rc::new(Node::new(node_type, String::from(b)));
                node_map.insert(b.to_owned(), Rc::clone(&node));
                node
            },
            Some(n) => Rc::clone(&n),
        };

        make_connection(node_a, node_b);
    }
    
    let start = node_map.get("start").unwrap();

    let paths = start.traverse(vec![], true, true);

    paths.len()
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
