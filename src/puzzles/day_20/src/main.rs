use core::panic;
use std::collections::HashMap;

use utils::read_lines;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pixel {
    Empty,
    Full,
}

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    data: Vec<Vec<Pixel>>,
}

struct ParsedTile {
    tile: Tile,
    borders: Vec<Vec<Pixel>>,
}

impl Tile {
    pub fn parse_borders(&self) -> ParsedTile {
        let top = self.data[0].clone();
        let bottom = self.data[self.data.len() - 1].clone();
        let left = self.data.iter().map(|r| r[0]).collect::<Vec<_>>();
        let right = self.data.iter().map(|r| r[r.len() - 1]).collect::<Vec<_>>();

        let mut top_rev = top.clone();
        top_rev.reverse();

        let mut bottom_rev = bottom.clone();
        bottom_rev.reverse();

        let mut left_rev = left.clone();
        left_rev.reverse();

        let mut right_rev = right.clone();
        right_rev.reverse();

        ParsedTile {
            tile: self.clone(),
            borders: vec![
                top, bottom, left, right, top_rev, bottom_rev, left_rev, right_rev,
            ],
        }
    }
}

type AdjacencyList = HashMap<usize, Vec<usize>>;

fn get_tile_adjecencies(target: &ParsedTile, other_tiles: &Vec<ParsedTile>) -> Vec<usize> {
    let mut result = vec![];

    for other_tile in other_tiles.iter() {
        if other_tile.tile.id != target.tile.id {
            for other_border in other_tile.borders.iter() {
                for border in target.borders.iter() {
                    if other_border == border {
                        result.push(other_tile.tile.id.clone());
                    }
                }
            }
        }
    }

    result
}

fn adjacencies(arr: Vec<Tile>) -> AdjacencyList {
    let parsed_tiles = arr
        .into_iter()
        .map(|t| t.parse_borders())
        .collect::<Vec<_>>();

    parsed_tiles
        .iter()
        .map(|parsed_tile| {
            let mut adjs = get_tile_adjecencies(parsed_tile, &parsed_tiles);
            adjs.sort();
            adjs.dedup();
            (parsed_tile.tile.id.clone(), adjs)
        })
        .collect()
}

fn get_corners(tiles: Vec<Tile>) -> Vec<usize> {
    let adjs = adjacencies(tiles);
    adjs.iter()
        .filter_map(|(k, v)| if v.len() == 2 { Some(k.clone()) } else { None })
        .collect()
}

fn solve_p1(tiles: Vec<Tile>) -> usize {
    let corners = get_corners(tiles);
    let mut result = 1;
    for corner_id in corners.iter() {
        result *= corner_id;
    }
    result
}

fn parse_input(lines: Vec<String>) -> Vec<Tile> {
    lines
        .split(|l| l == "")
        .map(|block| {
            let v = block.to_vec();
            let head = v.first().unwrap();
            let tail = &v[1..];
            let data = tail
                .iter()
                .map(|r| {
                    r.chars()
                        .map(|c| match c {
                            '.' => Pixel::Empty,
                            '#' => Pixel::Full,
                            _ => panic!("Invalid char"),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            let mut h = head.chars().skip(5).collect::<Vec<_>>();
            h.remove(h.len() - 1);
            let id = h.into_iter().collect::<String>().parse::<usize>().unwrap();

            Tile { data, id }
        })
        .collect()
}

fn main() {
    let lines = read_lines("src/inputs/day_20.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();
    let tiles = parse_input(lines);
    let result = solve_p1(tiles);

    println!("p1 result {:?}", result);
}

#[cfg(test)]
mod test {
    use utils::read_lines;

    use super::*;

    #[test]
    fn sample_input() {
        let lines = read_lines("./sample.txt")
            .unwrap()
            .flatten()
            .collect::<Vec<_>>();
        let tiles = parse_input(lines);
        let result = solve_p1(tiles);

        assert_eq!(result, 20899048083289);
    }
}
