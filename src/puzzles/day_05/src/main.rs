use std::isize;
use utils::read_lines;

fn main() {
    let lines = read_lines("src/inputs/day_05.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let ids = lines.into_iter().map(|line| {
        let BoardingPass { col, row } = BoardingPass::from_str(line);
        row * 8 + col
    });

    let max_id = ids.clone().max().unwrap();
    println!("Part 1: {:?}", max_id);

    let min_id = ids.clone().min().unwrap();
    let ids = ids.collect::<Vec<_>>();
    let maybe_found = (min_id..=max_id)
        .find(|id| !ids.contains(id) && ids.contains(&(*id + 1)) && ids.contains(&(*id - 1)));
    println!("Part 2: {:?}", maybe_found);
}

#[derive(PartialEq, Eq, Debug)]
pub struct BoardingPass {
    col: isize,
    row: isize,
}

impl BoardingPass {
    pub fn from_str<T: AsRef<str>>(s: T) -> BoardingPass {
        let (row, col) = s.as_ref().split_at(7);
        let transform = |str: &str, one: char| -> String {
            str.chars()
                .map(|c| if c == one { '1' } else { '0' })
                .collect::<String>()
        };

        let row = transform(row, 'B');
        let col = transform(col, 'R');

        BoardingPass {
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
        assert_eq!(BoardingPass::from_str("BFFFBBFRRR"), BoardingPass { row: 70, col: 7 });
        assert_eq!(BoardingPass::from_str("FFFBBBFRRR"), BoardingPass { row: 14, col: 7 });
        assert_eq!(BoardingPass::from_str("BBFFBBFRLL"), BoardingPass { row: 102, col: 4 });
    }
}
