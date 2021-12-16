use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone)]
struct BoardCell {
    value: u32,
    marked: bool,
}

impl BoardCell {
    fn new(value: u32) -> BoardCell {
        BoardCell {
            value: value,
            marked: false,
        }
    }

    fn get_value(self: BoardCell) -> u32 {
        self.value
    }

    fn mark(self: &mut BoardCell) {
        self.marked = true;
    }

    fn is_marked(self: BoardCell) -> bool {
        self.marked
    }
}

#[derive(Debug, Copy, Clone)]
struct Board {
    cells: [BoardCell; 25],
}

impl Board {
    pub fn new(values: [u32; 25]) -> Board {
        Board {
            cells: values.map(|value| BoardCell::new(value)),
        }
    }

    pub fn mark_cell(self: &mut Board, value: u32) {
        for cell in self.cells.iter_mut() {
            if cell.get_value() == value {
                cell.mark();
            }
        }
    }

    pub fn get_board_cell(self: Board, row: usize, col: usize) -> BoardCell {
        self.get_board_cell_by_index(row * 5 + col)
    }

    pub fn get_board_cell_by_index(self: Board, index: usize) -> BoardCell {
        self.cells[index]
    }

    pub fn has_won(self: Board) -> Option<u32> {
        let mut ret = false;

        let mut c = 0;
        while c < 5 && !ret {
            let mut potential = true;
            // check columns for win
            for r in 0..5 {
                let cell = self.get_board_cell(r, c);
                potential = cell.is_marked();
                if !potential {
                    break;
                }
            }
            ret = potential;
            c = c + 1;
        }

        // check rows for win
        let mut r = 0;
        while r < 5 && !ret {
            let mut potential = true;
            // check columns for win
            for c in 0..5 {
                let cell = self.get_board_cell(r, c);
                potential = cell.is_marked();
                if !potential {
                    break;
                }
            }
            ret = potential;
            r = r + 1;
        }

        if ret {
            let val = self
                .cells
                .into_iter()
                .filter(|c| !c.is_marked())
                .fold(0, |acc, c| acc + c.get_value());
            Some(val)
        } else {
            None
        }
    }
}

fn problem1(lines: &mut std::str::Lines) -> Option<u32> {
    let mut picked_values_split = lines.next().unwrap().split(',');

    let mut boards: Vec<Board> = Vec::new();

    // first read blank line separating boards; then read 5 lines.
    // if no blank line then input is over
    while lines.next().is_some() {
        // join together 5 lines for n
        let board_lines_iter = lines.take(5);

        let mut board_values: [u32; 25] = [0; 25];
        let mut i = 0;
        for line in board_lines_iter {
            for value in line.split_whitespace() {
                let num = value.parse::<u32>().unwrap();
                board_values[i] = num;
                i = i + 1;
            }
        }

        let board = Board::new(board_values);
        boards.push(board);
    }

    let winning_board: Option<Board> = None;
    while winning_board.is_none() {
        // mark next value on boards
        let next_value = picked_values_split.next().unwrap().parse::<u32>().unwrap();

        let boards_iter = boards.iter_mut();

        for board in boards_iter {
            board.mark_cell(next_value);
            let has_won = board.has_won();
            if has_won.is_some() {
                return Some(has_won.unwrap() * next_value);
            }
        }
    }

    None
}

fn problem2(lines: &mut std::str::Lines) -> Option<u32> {
    let mut picked_values_split = lines.next().unwrap().split(',');

    let mut boards: Vec<Board> = Vec::new();

    // first read blank line separating boards; then read 5 lines.
    // if no blank line then input is over
    while lines.next().is_some() {
        // join together 5 lines for n
        let board_lines_iter = lines.take(5);

        let mut board_values: [u32; 25] = [0; 25];
        let mut i = 0;
        for line in board_lines_iter {
            for value in line.split_whitespace() {
                let num = value.parse::<u32>().unwrap();
                board_values[i] = num;
                i = i + 1;
            }
        }

        let board = Board::new(board_values);
        boards.push(board);
    }

    while boards.len() > 0 {
        // mark next value on boards
        let next_value = picked_values_split.next().unwrap().parse::<u32>().unwrap();

        let boards_iter = boards.iter_mut();

        let mut indices_to_remove: Vec<usize> = Vec::new();
        let mut index = 0;
        for board in boards_iter {
            board.mark_cell(next_value);
            let has_won = board.has_won();
            if has_won.is_some() {
                indices_to_remove.push(index);
            }
            index = index + 1;
        }

        if boards.len() == 1 {
            let board = boards[0];
            if board.has_won().is_some() {
                return Some(board.has_won().unwrap() * next_value);
            }
        }

        for i in indices_to_remove.iter().rev() {
            boards.remove(*i);
        }
    }

    None
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
    println!("Problem1: {}", problem1(&mut lines.clone()).unwrap_or(0));
    println!("Problem2: {}", problem2(&mut lines.clone()).unwrap_or(0));
}
