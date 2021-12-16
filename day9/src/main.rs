use std::fs::File;
use std::io::prelude::*;

struct Heightmap {
    cols: usize,
    rows: usize,
    heights: Vec<u32>,
}

impl Heightmap {
    pub fn new(rows: Vec<Vec<u32>>) -> Heightmap {
        let mut heights: Vec<u32> = Vec::new();

        let num_rows = rows.len();
        let num_cols = rows.get(0).unwrap().len();
        for col in rows {
            for cell in col {
                heights.push(cell);
            }
        }

        Heightmap {
            cols: num_cols,
            rows: num_rows,
            heights: heights,
        }
    }

    pub fn get_low_point_indices(self: &Heightmap) -> Vec<usize> {
        let mut low_point_indices: Vec<usize> = Vec::new();

        let height_iterator = self.heights.iter();
        for (index, height) in height_iterator.enumerate() {
            let neighbor_heights = self.get_neighbor_heights(index);

            if neighbor_heights.iter().all(|nh| height < nh) {
                low_point_indices.push(index);
            }
        }

        low_point_indices
    }

    pub fn get_low_point_values(self: &Heightmap) -> Vec<u32> {
        let mut low_point_values: Vec<u32> = Vec::new();

        let height_iterator = self.heights.iter();
        for (index, height) in height_iterator.enumerate() {
            let neighbor_heights = self.get_neighbor_heights(index);

            if neighbor_heights.iter().all(|nh| height < nh) {
                low_point_values.push(*height);
            }
        }

        low_point_values
    }

    fn convert_index_to_pos(cols: usize, index: usize) -> (usize, usize) {
        (index / cols, index % cols)
    }

    fn convert_pos_to_index(cols: usize, pos: (usize, usize)) -> usize {
        cols * pos.0 + pos.1
    }

    fn get_neighbors_pos(self: &Heightmap, index: usize) -> Vec<usize> {
        let mut neighor_pos: Vec<usize> = Vec::new();

        let (r, c) = Heightmap::convert_index_to_pos(self.cols, index);
        let mut potential_neighbor_positions: Vec<(usize, usize)> = Vec::new();

        if r != 0 {
            potential_neighbor_positions.push((r - 1, c));
        }

        if c != 0 {
            potential_neighbor_positions.push((r, c - 1));
        }

        if c != (self.cols - 1) {
            potential_neighbor_positions.push((r, c + 1));
        }

        if r != (self.rows - 1) {
            potential_neighbor_positions.push((r + 1, c));
        }

        for pos in potential_neighbor_positions {
            let (r, c) = (pos.0, pos.1);
            let index = Heightmap::convert_pos_to_index(self.cols, (r, c));
            if let Some(_) = self.heights.get(index) {
                neighor_pos.push(index);
            }
        }

        neighor_pos
    }

    fn get_neighbor_heights(self: &Heightmap, index: usize) -> Vec<u32> {
        let mut neighbor_heights: Vec<u32> = Vec::new();

        self.get_neighbors_pos(index).iter().for_each(|&i| {
            let val = self.heights.get(i).unwrap();
            neighbor_heights.push(*val);
        });

        neighbor_heights
    }

    pub fn get_height_pos_in_basin(self: &Heightmap, index: usize) -> Vec<usize> {
        let mut ret: Vec<usize> = Vec::new();
        ret.push(index);
        ret.append(&mut self.get_higher_heights(index));
        ret.sort();
        ret.dedup();
        ret
    }

    fn get_higher_heights(self: &Heightmap, index: usize) -> Vec<usize> {
        let height = self.heights.get(index).unwrap();
        let mut pos: Vec<usize> = self
            .get_neighbors_pos(index)
            .into_iter()
            .filter(|&i| {
                let neighbor_height = self.heights.get(i).unwrap();
                height < neighbor_height && neighbor_height != &9u32
            })
            .collect();

        if pos.len() == 0 {
            return pos;
        }

        let mut child_pos = pos
            .iter()
            .map(|&p| self.get_higher_heights(p))
            .flatten()
            .collect::<Vec<usize>>();

        pos.append(&mut child_pos);

        pos
    }
}

fn problem1(lines: &mut std::str::Lines) -> u32 {
    let mut values: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut row: Vec<u32> = Vec::new();
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap());
        }
        values.push(row);
    }

    let heightmap = Heightmap::new(values);
    heightmap
        .get_low_point_values()
        .iter()
        .fold(0, |acc, cur| acc + cur + 1)
}

fn problem2(lines: &mut std::str::Lines) -> u32 {
    let mut values: Vec<Vec<u32>> = Vec::new();
    for line in lines {
        let mut row: Vec<u32> = Vec::new();
        for ch in line.chars() {
            row.push(ch.to_digit(10).unwrap());
        }
        values.push(row);
    }

    let heightmap = Heightmap::new(values);
    let low_point_index = heightmap.get_low_point_indices();
    let mut basin_sizes: Vec<usize> = low_point_index
        .into_iter()
        .map(|i| {
            let h = heightmap.get_height_pos_in_basin(i);
            h.len()
        })
        .collect();

    basin_sizes.sort();
    basin_sizes.reverse();

    (basin_sizes.get(0).unwrap() * basin_sizes.get(1).unwrap() * basin_sizes.get(2).unwrap()) as u32
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
