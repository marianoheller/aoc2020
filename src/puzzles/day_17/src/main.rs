use std::{collections::HashMap, hash::Hash};

use utils::read_lines;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CubeState {
    Active,
    Inactive,
}

pub trait SpaceCoords {
    fn for_each_neighbor<F>(&self, f: F)
    where
        F: FnMut(&Self);
}

// ========================================================================
// 3D
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Coords3D {
    x: isize,
    y: isize,
    z: isize,
}

impl From<(isize, isize)> for Coords3D {
    fn from(tuple: (isize, isize)) -> Self {
        Coords3D {
            x: tuple.0,
            y: tuple.1,
            z: 0,
        }
    }
}

impl From<(isize, isize, isize)> for Coords3D {
    fn from(tuple: (isize, isize, isize)) -> Self {
        Coords3D {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl SpaceCoords for Coords3D {
    fn for_each_neighbor<F>(&self, mut f: F)
    where
        F: FnMut(&Coords3D),
    {
        for x in (self.x - 1)..=(self.x + 1) {
            for y in (self.y - 1)..=(self.y + 1) {
                for z in (self.z - 1)..=(self.z + 1) {
                    let coords: Coords3D = Coords3D::from((x, y, z));
                    f(&coords);
                }
            }
        }
    }
}

// ========================================================================
// 4D
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Coords4D {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl From<(isize, isize)> for Coords4D {
    fn from(tuple: (isize, isize)) -> Self {
        Coords4D {
            x: tuple.0,
            y: tuple.1,
            z: 0,
            w: 0,
        }
    }
}

impl From<(isize, isize, isize, isize)> for Coords4D {
    fn from(tuple: (isize, isize, isize, isize)) -> Self {
        Coords4D {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
            w: tuple.3,
        }
    }
}

impl SpaceCoords for Coords4D {
    fn for_each_neighbor<F>(&self, mut f: F)
    where
        F: FnMut(&Coords4D),
    {
        for x in (self.x - 1)..=(self.x + 1) {
            for y in (self.y - 1)..=(self.y + 1) {
                for z in (self.z - 1)..=(self.z + 1) {
                    for w in (self.w - 1)..=(self.w + 1) {
                        let coords: Coords4D = Coords4D::from((x, y, z, w));
                        f(&coords);
                    }
                }
            }
        }
    }
}

// ========================================================================

#[derive(Debug)]
pub struct NeighborsCount {
    active: usize,
    inactive: usize,
}

impl NeighborsCount {
    fn new() -> Self {
        NeighborsCount {
            active: 0,
            inactive: 0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct World<T>
where
    T: Clone + Hash + Eq + From<(isize, isize)> + SpaceCoords,
{
    data: HashMap<T, CubeState>,
}

impl<T> World<T>
where
    T: Clone + Hash + Eq + From<(isize, isize)> + SpaceCoords,
{
    pub fn tick(&mut self) {
        self.pad();
        let og_world = self.clone();
        for coords in og_world.data.keys() {
            let count = World::count_neighbors(coords, &og_world);
            let cube = self.data.get_mut(coords).unwrap();

            match cube {
                CubeState::Active if count.active == 2 || count.active == 3 => {
                    *cube = CubeState::Active;
                }
                CubeState::Active => {
                    *cube = CubeState::Inactive;
                }
                CubeState::Inactive if count.active == 3 => {
                    *cube = CubeState::Active;
                }
                _ => {}
            }
        }
    }

    fn pad(&mut self) {
        for c in self.data.clone().keys() {
            c.for_each_neighbor(|cn| {
                let neighbor = self.data.get(&cn);
                match neighbor {
                    None => {
                        self.data.insert(cn.clone(), CubeState::Inactive);
                    }
                    _ => {}
                };
            });
        }
    }

    pub fn count_active(&self) -> usize {
        self.data.values().fold(0, |mut acc, v| {
            if *v == CubeState::Active {
                acc += 1;
            }
            acc
        })
    }

    fn count_neighbors(c: &T, world: &World<T>) -> NeighborsCount {
        let mut neighbors_count = NeighborsCount::new();

        c.for_each_neighbor(|cn| {
            if cn != c {
                let neighbor = world.data.get(&cn);
                match neighbor {
                    Some(CubeState::Active) => {
                        neighbors_count.active += 1;
                    }
                    _ => {
                        neighbors_count.inactive += 1;
                    }
                }
            }
        });

        neighbors_count
    }

    pub fn from_lines<S: AsRef<str>>(lines: &Vec<S>) -> World<T> {
        let data = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.as_ref()
                    .char_indices()
                    .map(|(x, c)| {
                        let state = match c {
                            '#' => CubeState::Active,
                            '.' => CubeState::Inactive,
                            _ => {
                                panic!("Invalid character")
                            }
                        };
                        ((x as isize, y as isize).into(), state)
                    })
                    .collect::<Vec<(_, CubeState)>>()
            })
            .collect();

        World { data }
    }
}

fn main() {
    let lines = read_lines("src/inputs/day_17.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let mut world = World::<Coords3D>::from_lines(&lines);
    for _i in 0..6 {
        world.tick();
    }
    println!("Part 1: {:?}", world.count_active());
    
    
    let mut world = World::<Coords4D>::from_lines(&lines);
    for _i in 0..6 {
        world.tick();
    }
    println!("Part 2: {:?}", world.count_active());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let mut world = World::<Coords3D> {
            data: vec![
                ((0, 0, 0).into(), CubeState::Inactive),
                ((1, 0, 0).into(), CubeState::Active),
                ((2, 0, 0).into(), CubeState::Inactive),
                ((0, 1, 0).into(), CubeState::Inactive),
                ((1, 1, 0).into(), CubeState::Inactive),
                ((2, 1, 0).into(), CubeState::Active),
                ((0, 2, 0).into(), CubeState::Active),
                ((1, 2, 0).into(), CubeState::Active),
                ((2, 2, 0).into(), CubeState::Active),
            ]
            .into_iter()
            .collect(),
        };

        // First cycle
        world.tick();
        assert_eq!(world.count_active(), 11);

        // Second cycle
        world.tick();
        assert_eq!(world.count_active(), 21);

        // Third cycle
        world.tick();
        assert_eq!(world.count_active(), 38);

        for _i in 0..3 {
            world.tick();
        }

        assert_eq!(world.count_active(), 112);
    }
}
