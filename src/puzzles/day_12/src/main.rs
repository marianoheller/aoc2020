use std::f32;

use utils::read_lines;

struct Ship {
    pub pos: (i32, i32),
    pub angle: i32,
    pub pos_rel_wp: (i32, i32),
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            pos: (0, 0),
            angle: 0,
            pos_rel_wp: (10, 1),
        }
    }

    pub fn mov(&mut self, delta_pos: (i32, i32)) {
        self.pos.0 += delta_pos.0;
        self.pos.1 += delta_pos.1;
    }

    pub fn rot(&mut self, angle: i32) {
        self.angle += angle;
    }

    pub fn forward(&mut self, val: i32) {
        let angle = (self.angle as f32) * f32::consts::PI / 180.0;
        let x_comp = angle.cos().round() as i32;
        let y_comp = angle.sin().round() as i32;

        self.mov((x_comp * val, y_comp * val));
    }

    pub fn mov_wp(&mut self, delta_pos: (i32, i32)) {
        self.pos_rel_wp.0 += delta_pos.0;
        self.pos_rel_wp.1 += delta_pos.1;
    }

    pub fn rot_wp(&mut self, angle: i32) {
        let (x1, y1) = (self.pos_rel_wp.0 as f32, self.pos_rel_wp.1 as f32);

        let rads = (angle as f32) * f32::consts::PI / 180.0;

        let x2 = rads.cos() * x1 - rads.sin() * y1;
        let y2 = rads.sin() * x1 + rads.cos() * y1;

        self.pos_rel_wp = (x2.round() as i32, y2.round() as i32);
    }

    pub fn forward_wp(&mut self, val: i32) {
        self.mov((self.pos_rel_wp.0 * val, self.pos_rel_wp.1 * val));
    }

    pub fn run_p1_line<T: AsRef<str>>(&mut self, s: T) {
        let mut it = s.as_ref().chars();

        let cmd = it.next().unwrap();
        let val = it.collect::<String>().parse::<i32>().unwrap();

        match cmd {
            'N' => self.mov((0, val)),
            'S' => self.mov((0, -val)),
            'E' => self.mov((val, 0)),
            'W' => self.mov((-val, 0)),
            'L' => self.rot(val),
            'R' => self.rot(-val),
            'F' => self.forward(val),
            _ => {
                panic!("Invalid cmd")
            }
        }
    }

    pub fn run_p2_line<T: AsRef<str>>(&mut self, s: T) {
        let mut it = s.as_ref().chars();

        let cmd = it.next().unwrap();
        let val = it.collect::<String>().parse::<i32>().unwrap();

        match cmd {
            'N' => self.mov_wp((0, val)),
            'S' => self.mov_wp((0, -val)),
            'E' => self.mov_wp((val, 0)),
            'W' => self.mov_wp((-val, 0)),
            'L' => self.rot_wp(val),
            'R' => self.rot_wp(-val),
            'F' => self.forward_wp(val),
            _ => {
                panic!("Invalid cmd")
            }
        }
    }
}

pub fn solve_p1<T: AsRef<str>>(input: &Vec<T>) -> i32 {
    let mut ship = Ship::new();
    input.iter().for_each(|v| {
        ship.run_p1_line(v);
    });

    ship.pos.0.abs() + ship.pos.1.abs()
}

pub fn solve_p2<T: AsRef<str>>(input: &Vec<T>) -> i32 {
    let mut ship = Ship::new();
    input.iter().for_each(|v| {
        ship.run_p2_line(v);
    });

    ship.pos.0.abs() + ship.pos.1.abs()
}

fn main() {
    let values = read_lines("src/inputs/day_12.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", solve_p1(&values));
    println!("Part 1: {:?}", solve_p2(&values));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_p1() {
        let input = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(solve_p1(&input), 25);
    }

    #[test]
    fn p1_with_rotation() {
        let input = vec!["F10", "N3", "F7", "R90", "F11", "R90", "R90", "R90", "R90"];
        assert_eq!(solve_p1(&input), 25);
    }

    #[test]
    fn p1_with_back_and_forth() {
        let input = vec![
            "F10", "N3", "F7", "R90", "F11", "N10", "E10", "W10", "S20", "N10",
        ];
        assert_eq!(solve_p1(&input), 25);
    }

    #[test]
    fn p1_going_back() {
        let input = vec![
            "F10", "N3", "F7", "R90", "F11", "L180", "F11", "L90", "F7", "S3", "F10",
        ];
        assert_eq!(solve_p1(&input), 0);
    }

    #[test]
    fn example_p2() {
        let input = vec!["F10", "N3", "F7", "R90", "F11"];
        assert_eq!(solve_p2(&input), 286);
    }

    #[test]
    fn p2_step1() {
        let input = vec!["F10"];
        assert_eq!(solve_p2(&input), 110);
    }

    #[test]
    fn p2_step2() {
        let input = vec!["F10", "N3"];
        assert_eq!(solve_p2(&input), 110);
    }

    #[test]
    fn p2_step3() {
        let input = vec!["F10", "N3", "F7"];
        assert_eq!(solve_p2(&input), 208);
    }

    #[test]
    fn p2_step4() {
        let input = vec!["F10", "N3", "F7", "R90"];
        assert_eq!(solve_p2(&input), 208);
    }
}
