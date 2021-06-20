use std::collections::HashMap;

use utils::read_lines;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CubeState {
    Active,
    Inactive,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Coords {
    x: isize,
    y: isize,
    z: isize,
}

impl Into<Coords> for (isize, isize, isize) {
    fn into(self) -> Coords {
        Coords {
            x: self.0,
            y: self.1,
            z: self.2,
        }
    }
}

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
pub struct World {
    data: HashMap<Coords, CubeState>,
}

impl World {
    pub fn tick(&mut self) {
        self.pad();
        let og_world: World = self.clone();
        for coords in og_world.data.keys() {
            let count = World::count_neighbours(coords, &og_world);
            let cube = self.data.get_mut(coords).unwrap();

            // println!("count {:?} for {:?}", count, coords);

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
            for x in (c.x - 1)..=(c.x + 1) {
                for y in (c.y - 1)..=(c.y + 1) {
                    for z in (c.z - 1)..=(c.z + 1) {
                        let coords: Coords = (x, y, z).into();
                        let neighbor = self.data.get(&coords);

                        match neighbor {
                            None => {
                                self.data.insert(coords, CubeState::Inactive);
                            }
                            _ => {}
                        };
                    }
                }
            }
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

    fn count_neighbours(c: &Coords, world: &World) -> NeighborsCount {
        let mut neighbors_count = NeighborsCount::new();

        for x in (c.x - 1)..=(c.x + 1) {
            for y in (c.y - 1)..=(c.y + 1) {
                for z in (c.z - 1)..=(c.z + 1) {
                    if (x, y, z) != (c.x, c.y, c.z) {
                        let coords: Coords = (x, y, z).into();
                        let neighbor = world.data.get(&coords);
                        match neighbor {
                            Some(CubeState::Active) => {
                                neighbors_count.active += 1;
                            }
                            _ => {
                                neighbors_count.inactive += 1;
                            }
                        }
                    }
                }
            }
        }

        neighbors_count
    }

    pub fn from_lines<T: AsRef<str>>(lines: &Vec<T>) -> World {
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
                        ((x as isize, y as isize, 0 as isize).into(), state)
                    })
                    .collect::<Vec<(Coords, CubeState)>>()
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

    let mut world: World = World::from_lines(&lines);

    for _i in 0..6 {
        world.tick();
    }

    println!("Part 1: {:?}", world.count_active());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let mut world = World {
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
