use utils::read_lines;

fn solve_p1(nums: Vec<i32>) -> Option<i32> {
    let mut result: Option<i32> = None;
    for num1 in nums.iter() {
        for num2 in nums.iter() {
            if num1 + num2 == 2020 && result.is_none() {
                result = Some(num1 * num2)
            }
        }
    }

    result
}
fn solve_p2(nums: Vec<i32>) -> Option<i32> {
    let mut result: Option<i32> = None;
    for num1 in nums.iter() {
        for num2 in nums.iter() {
            for num3 in nums.iter() {
                if num1 + num2 + num3 == 2020 && result.is_none() {
                    result = Some(num1 * num2 * num3)
                }
            }
        }
    }

    result
}

fn read_ints(path: &str) -> Vec<i32> {
    let mut out: Vec<i32> = Vec::new();
    let lines = read_lines(path).unwrap();

    for line in lines {
        let num = line.unwrap().parse::<i32>().unwrap();
        out.push(num);
    }

    out
}

fn main() {
    let nums = read_ints("src/inputs/day_01.txt");
    println!("Read {} nums", nums.len());

    let p1 = solve_p1(nums.clone());
    println!("Part 1: {:?}", p1);

    let p2 = solve_p2(nums.clone());
    println!("Part 2: {:?}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_p1() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve_p1(input.to_vec()), Some(514579));
    }

    #[test]
    fn example_p2() {
        let input = [1721, 979, 366, 299, 675, 1456];
        assert_eq!(solve_p2(input.to_vec()), Some(241861950));
    }
    
}
