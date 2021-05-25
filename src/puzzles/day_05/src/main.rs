use std::isize;
use utils::read_lines;

fn main() {
    let lines = read_lines("src/inputs/day_05.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let max_id = lines
        .into_iter()
        .map(|line| {
            let Seat { col, row } = Seat::from_str(line);
            row * 8 + col
        })
        .max()
        .unwrap();

    println!("Part 1: {:?}", max_id);
}

#[derive(PartialEq, Eq, Debug)]
pub struct Seat {
    col: isize,
    row: isize,
}

impl Seat {
    pub fn from_str<T: AsRef<str>>(s: T) -> Seat {
        let (row, col) = s.as_ref().split_at(7);
        let row = row
            .chars()
            .map(|c| if c == 'B' { '1' } else { '0' })
            .collect::<String>();
        let col = col
            .chars()
            .map(|c| if c == 'R' { '1' } else { '0' })
            .collect::<String>();

        Seat {
            col: isize::from_str_radix(&col[..], 2).unwrap(),
            row: isize::from_str_radix(&row[..], 2).unwrap(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn examples_p1() {
        assert_eq!(Seat::from_str("BFFFBBFRRR"), Seat { row: 70, col: 7 });
        assert_eq!(Seat::from_str("FFFBBBFRRR"), Seat { row: 14, col: 7 });
        assert_eq!(Seat::from_str("BBFFBBFRLL"), Seat { row: 102, col: 4 });
    }
}
