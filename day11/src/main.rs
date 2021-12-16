use std::fs::File;
use std::io::prelude::*;

struct Octopus {
    energy_level: u32,
    has_flashed: bool,
    neighbors: Vec<usize>
}

impl Octopus {
    pub fn new(energy_level: u32) -> Self {
        Octopus {
            energy_level,
            has_flashed: false,
            neighbors: Vec::new()
        }
    }

    pub fn add_neighbors(self: &mut Octopus, neighbors: Vec<usize>) {
        self.neighbors = neighbors;
    }

    pub fn increase_energy_level(self: &mut Octopus) {
        self.energy_level += 1;
    }

    pub fn try_flash(self: &mut Octopus) -> bool {
        if self.energy_level > 9 && !self.has_flashed {
            self.has_flashed = true;
            return true;
        }
        false
    }

    pub fn reset(self: &mut Octopus) {
        if self.has_flashed {
            self.energy_level = 0;
        }
        self.has_flashed = false
    }
}

fn problem1(lines: &mut std::str::Lines) -> u32 {
    let mut total_flashes: u32 = 0;
    let mut octopi: Vec<Octopus> = Vec::new();
    for line in lines {
        for char in line.chars() {
            let energy_level: u32 = char.to_digit(10).unwrap();
            let octopus = Octopus::new(energy_level);
            octopi.push(octopus);
        }
    }

    // assign neighbors; diagonals count
    let rows = 10;
    let cols = 10;
    for (index, octopus) in octopi.iter_mut().enumerate() {
        let index = index as usize;
        let (row, col) = (index / rows, index % cols);
        let mut neighbor_indices: Vec<usize> = vec![];

        if col > 0 {
            neighbor_indices.push(row * cols + col - 1);
        }

        if col < cols - 1 {
            neighbor_indices.push(row * cols + col + 1);
        }

        if row > 0 {
            neighbor_indices.push((row - 1) * cols + col);

            if col > 0 {
                neighbor_indices.push((row - 1) * cols + col - 1);
            }

            if col < cols - 1 {
                neighbor_indices.push((row - 1) * cols + col + 1);
            }
        }

        if row < rows - 1 {
            neighbor_indices.push((row + 1) * cols + col);

            if col > 0 {
                neighbor_indices.push((row + 1) * cols + col - 1);
            }

            if col < cols - 1 {
                neighbor_indices.push((row + 1) * cols + col + 1);
            }
        }
        octopus.add_neighbors(neighbor_indices);
    }

    let mut step = 1;
    while step <= 100 {
        for octopus in octopi.iter_mut() {
            octopus.increase_energy_level();
        }

        loop {
            let mut flash_occurred = false;
            let mut to_update: Vec<usize> = Vec::new();
            for octopus in octopi.iter_mut() {
                if octopus.try_flash() {
                    flash_occurred = true;
                    total_flashes += 1;
                    to_update.append(&mut octopus.neighbors.clone());
                }
            }

            if !flash_occurred {
                break;
            }

            for otu in to_update {
                octopi.get_mut(otu).unwrap().increase_energy_level();
            }
        }

        for octopus in octopi.iter_mut() {
            octopus.reset();
        }

        step += 1;
    }
    
    total_flashes
}

fn problem2(lines: &mut std::str::Lines) -> u32 {
    let mut octopi: Vec<Octopus> = Vec::new();
    for line in lines {
        for char in line.chars() {
            let energy_level: u32 = char.to_digit(10).unwrap();
            let octopus = Octopus::new(energy_level);
            octopi.push(octopus);
        }
    }

    // assign neighbors; diagonals count
    let rows = 10;
    let cols = 10;
    for (index, octopus) in octopi.iter_mut().enumerate() {
        let index = index as usize;
        let (row, col) = (index / rows, index % cols);
        let mut neighbor_indices: Vec<usize> = vec![];

        if col > 0 {
            neighbor_indices.push(row * cols + col - 1);
        }

        if col < cols - 1 {
            neighbor_indices.push(row * cols + col + 1);
        }

        if row > 0 {
            neighbor_indices.push((row - 1) * cols + col);

            if col > 0 {
                neighbor_indices.push((row - 1) * cols + col - 1);
            }

            if col < cols - 1 {
                neighbor_indices.push((row - 1) * cols + col + 1);
            }
        }

        if row < rows - 1 {
            neighbor_indices.push((row + 1) * cols + col);

            if col > 0 {
                neighbor_indices.push((row + 1) * cols + col - 1);
            }

            if col < cols - 1 {
                neighbor_indices.push((row + 1) * cols + col + 1);
            }
        }
        octopus.add_neighbors(neighbor_indices);
    }

    let mut step = 1;
    loop {
        for octopus in octopi.iter_mut() {
            octopus.increase_energy_level();
        }

        let mut flashes_occurred = 0;
        loop {
            let mut flash_occurred = false;
            let mut to_update: Vec<usize> = Vec::new();
            for octopus in octopi.iter_mut() {
                if octopus.try_flash() {
                    flashes_occurred += 1;
                    flash_occurred = true;
                    to_update.append(&mut octopus.neighbors.clone());
                }
            }

            if flashes_occurred == 100 {
                return step;
            }

            if !flash_occurred {
                break;
            }

            for otu in to_update {
                octopi.get_mut(otu).unwrap().increase_energy_level();
            }
        }

        for octopus in octopi.iter_mut() {
            octopus.reset();
        }

        step += 1;
    }
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
