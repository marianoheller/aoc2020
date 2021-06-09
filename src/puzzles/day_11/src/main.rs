use std::cmp::{max, min};

use utils::read_lines;

fn main() {
    let values = read_lines("src/inputs/day_11.txt")
        .unwrap()
        .flatten()
        .map(|line| {
            line.split("")
                .into_iter()
                .filter(|v| *v != "")
                .map(|v| v.to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let initial_layout = parse(&values);

    println!("Part 1: {:?}", solve_p1(&initial_layout));
    println!("Part 2: {:?}", solve_p2(&initial_layout));
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Cell {
    Empty,
    Occupied,
    Floor,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SeatContext {
    empty: usize,
    occupied: usize,
    floor: usize,
}

pub fn parse<T: AsRef<str>>(grid: &Vec<Vec<T>>) -> Vec<Vec<Cell>> {
    grid.into_iter()
        .map(|row| {
            row.into_iter()
                .map(|cell| match cell.as_ref() {
                    "L" => Cell::Empty,
                    "#" => Cell::Occupied,
                    _ => Cell::Floor,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn count_ctx_p1(grid: &Vec<Vec<Cell>>, row_index: usize, col_index: usize) -> SeatContext {
    let max_row_index = grid.len() - 1;
    let max_col_index = grid[0].len() - 1;

    let low_row_index = max((row_index as i32) - 1, 0) as usize;
    let high_row_index = min((row_index as i32) + 1, max_row_index as i32) as usize;

    let low_col_index = max((col_index as i32) - 1, 0) as usize;
    let high_col_index = min((col_index as i32) + 1, max_col_index as i32) as usize;

    let mut empty = 0;
    let mut occupied = 0;
    let mut floor = 0;

    for row in low_row_index..=high_row_index {
        for col in low_col_index..=high_col_index {
            if (row, col) != (row_index, col_index) {
                match grid[row][col] {
                    Cell::Empty => {
                        empty += 1;
                    }
                    Cell::Occupied => {
                        occupied += 1;
                    }
                    Cell::Floor => {
                        floor += 1;
                    }
                }
            }
        }
    }

    SeatContext {
        empty,
        occupied,
        floor,
    }
}

pub fn count_ctx_p2(grid: &Vec<Vec<Cell>>, row_index: usize, col_index: usize) -> SeatContext {
    let mut empty = 0;
    let mut occupied = 0;
    let mut floor = 0;

    let directions: Vec<(i32, i32)> = vec![
        (-1, 0),
        (1, 0),
        (0, 1),
        (0, -1),
        (-1, -1),
        (1, 1),
        (-1, 1),
        (1, -1),
    ];

    directions.into_iter().for_each(|(row_inc, col_inc)| {
        let mut result = None;

        let mut new_row = (row_index as i32) + row_inc;
        let mut new_col = (col_index as i32) + col_inc;

        while let (Some(c), None) = (
            grid.get(new_row as usize)
                .and_then(|row| row.get(new_col as usize)),
            result,
        ) {
            if *c != Cell::Floor {
                result = Some(*c);
            }

            new_row += row_inc;
            new_col += col_inc;
        }

        if let Some(c) = result {
            match c {
                Cell::Empty => {
                    empty += 1;
                }
                Cell::Occupied => {
                    occupied += 1;
                }
                Cell::Floor => {
                    floor += 1;
                }
            }
        }
    });

    SeatContext {
        empty,
        occupied,
        floor,
    }
}

pub fn tick_p1(grid: &mut Vec<Vec<Cell>>) {
    let original = grid.clone();
    for (row_index, row) in original.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            let ctx = count_ctx_p1(&original, row_index, col_index);
            match cell {
                Cell::Empty if ctx.occupied == 0 => grid[row_index][col_index] = Cell::Occupied,
                Cell::Occupied if ctx.occupied >= 4 => grid[row_index][col_index] = Cell::Empty,
                _ => {}
            }
        }
    }
}

pub fn tick_p2(grid: &mut Vec<Vec<Cell>>) {
    let original = grid.clone();
    for (row_index, row) in original.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            let ctx = count_ctx_p2(&original, row_index, col_index);
            match cell {
                Cell::Empty if ctx.occupied == 0 => grid[row_index][col_index] = Cell::Occupied,
                Cell::Occupied if ctx.occupied >= 5 => grid[row_index][col_index] = Cell::Empty,
                _ => {}
            }
        }
    }
}

pub fn count_occupied(grid: &Vec<Vec<Cell>>) -> u32 {
    let mut result = 0;
    for row in grid.iter() {
        for cell in row.iter() {
            match cell {
                Cell::Occupied => result += 1,
                _ => {}
            }
        }
    }

    result
}

pub fn solve_p1(_grid: &Vec<Vec<Cell>>) -> u32 {
    let mut grid = _grid.clone();
    let mut prev_grid = vec![vec![]];

    while prev_grid != grid {
        prev_grid = grid.clone();
        tick_p1(&mut grid);
    }

    count_occupied(&grid)
}

pub fn solve_p2(_grid: &Vec<Vec<Cell>>) -> u32 {
    let mut grid = _grid.clone();
    let mut prev_grid = vec![vec![]];

    while prev_grid != grid {
        prev_grid = grid.clone();
        tick_p2(&mut grid);
    }

    count_occupied(&grid)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let initial_layout = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .into_iter()
        .map(|row| {
            row.split("")
                .into_iter()
                .filter(|v| *v != "")
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

        let initial_layout = parse(&initial_layout);

        assert_eq!(solve_p1(&initial_layout), 37);
    }

    #[test]
    fn example_p2() {
        let initial_layout = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .into_iter()
        .map(|row| {
            row.split("")
                .into_iter()
                .filter(|v| *v != "")
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

        let initial_layout = parse(&initial_layout);

        assert_eq!(solve_p2(&initial_layout), 26);
    }
}
