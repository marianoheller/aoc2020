use utils::read_lines;
fn main() {
    let lines = read_lines("src/inputs/day_03.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let p1 = count_trees(&lines, 3, 1);
    println!("Part 1: {:?}", p1);

    let p2 = multiple_tree_count(&lines, &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]);
    println!("Part 2: {:?}", p2);
}

fn count_trees<S: AsRef<str>>(lines: &Vec<S>, slope_h: usize, slope_v: usize) -> usize {
    let width = lines[0].as_ref().len();
    let (count, _) = lines
        .into_iter()
        .enumerate()
        .filter(|&(i, _)| i % slope_v == 0)
        .fold((0, 0), |(count, col), (_, line)| {
            let level = line.as_ref().chars().into_iter().collect::<Vec<_>>();
            let maybe_tree = level[col % width];
            let new_count = if maybe_tree == '#' { count + 1 } else { count };
            (new_count, col + slope_h)
        });

    count
}

fn multiple_tree_count<S: AsRef<str>>(lines: &Vec<S>, slopes: &[(usize, usize)]) -> usize {
    slopes.into_iter().fold(1, |acc, (slope_h, slope_v)| {
        acc * count_trees(lines, *slope_h, *slope_v)
    })
}

#[cfg(test)]
mod test {
    use super::*;

    static INPUT: [&str; 11] = [
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    #[test]
    fn example_p1() {
        assert_eq!(count_trees(&INPUT.to_vec(), 3, 1), 7);
    }

    #[test]
    fn example_p2_1() {
        assert_eq!(count_trees(&INPUT.to_vec(), 1, 1), 2);
    }

    #[test]
    fn example_p2_2() {
        assert_eq!(count_trees(&INPUT.to_vec(), 5, 1), 3);
    }

    #[test]
    fn example_p2_3() {
        assert_eq!(count_trees(&INPUT.to_vec(), 7, 1), 4);
    }

    #[test]
    fn example_p2_4() {
        assert_eq!(count_trees(&INPUT.to_vec(), 1, 2), 2);
    }

    #[test]
    fn example_p2_5() {
        assert_eq!(
            multiple_tree_count(&INPUT.to_vec(), &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]),
            336
        );
    }
}
