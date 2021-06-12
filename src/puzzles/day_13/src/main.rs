use utils::read_lines;

fn main() {
    let lines = read_lines("src/inputs/day_13.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let t = lines[0].parse::<usize>().unwrap();
    let buses = lines[1]
        .split(",")
        .map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let buses_p1 = buses
        .clone()
        .iter()
        .filter(|b| b.is_some())
        .map(|b| b.unwrap())
        .collect();

    println!("Part 1: {:?}", solve_p1(t, &buses_p1));
    println!("Part 2: {:?}", solve_p2(&buses));
}

pub fn solve_p1(t: usize, buses: &Vec<usize>) -> usize {
    let (i, v) = buses
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let t = t as f32;
            let v = *v as f32;
            let rem = t % v;
            (i, (v - rem) as usize)
        })
        .min_by_key(|(_, v)| *v)
        .unwrap();

    let bus_id = buses[i];

    bus_id * v
}

pub fn solve_p2(buses: &Vec<Option<usize>>) -> usize {
    let after: usize = 100000000000000;
    let parsed = buses
        .iter()
        .enumerate()
        .filter_map(|(index, maybe_id)| maybe_id.map(|id| (id, index)))
        .collect::<Vec<_>>();

    parsed
        .iter()
        .fold((after, 1), |(base_timestamp, period), (id, offset)| {
            (0..)
                .find_map(|i| {
                    let timestamp = base_timestamp + i * period;
                    if (timestamp + offset) % id == 0 {
                        Some((timestamp, period * id))
                    } else {
                        None
                    }
                })
                .unwrap()
        })
        .0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn example_p1() {
        let t: usize = 939;
        let vals: Vec<usize> = vec![7, 13, 59, 31, 19];

        assert_eq!(solve_p1(t, &vals), 295);
    }

    #[test]
    pub fn examples_p2() {
        let vals: Vec<Option<usize>> = vec![Some(17), None, Some(13), Some(19)];
        assert_eq!(solve_p2(&vals), 3417);

        let vals: Vec<Option<usize>> = vec![Some(67), Some(7), Some(59), Some(61)];
        assert_eq!(solve_p2(&vals), 754018);

        let vals: Vec<Option<usize>> = vec![Some(67), None, Some(7), Some(59), Some(61)];
        assert_eq!(solve_p2(&vals), 779210);

        let vals: Vec<Option<usize>> = vec![Some(67), Some(7), None, Some(59), Some(61)];
        assert_eq!(solve_p2(&vals), 1261476);

        let vals: Vec<Option<usize>> = vec![Some(1789), Some(37), Some(47), Some(1889)];
        assert_eq!(solve_p2(&vals), 1202161486);
    }
}
