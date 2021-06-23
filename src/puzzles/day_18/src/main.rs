extern crate nom;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::char,
    character::complete::{digit1 as digit, space0 as space},
    combinator::map_res,
    multi::fold_many0,
    sequence::{delimited, pair},
    IResult,
};
use utils::read_lines;

use std::str::FromStr;

// We parse any expr surrounded by parens, ignoring all whitespaces around those
fn parens(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), expr, tag(")")), space)(i)
}

fn parens_v2(i: &str) -> IResult<&str, i64> {
    delimited(space, delimited(tag("("), expr_v2, tag(")")), space)(i)
}

// We transform an integer string into a i64, ignoring surrounding whitespaces
// We look for a digit suite, and try to convert it.
// If either str::from_utf8 or FromStr::from_str fail,
// we fallback to the parens parser defined above
fn factor(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        parens,
    ))(i)
}

fn factor_v2(i: &str) -> IResult<&str, i64> {
    alt((
        map_res(delimited(space, digit, space), FromStr::from_str),
        parens_v2,
    ))(i)
}

// We read an initial factor and for each time we find
// a * or / operator followed by another factor, we do
// the math by folding everything
fn term(i: &str) -> IResult<&str, i64> {
    let (i, init) = factor(i)?;

    fold_many0(
        pair(alt((char('*'), char('/'), char('+'), char('-'))), factor),
        init,
        |acc, (op, val): (char, i64)| match op {
            '*' => acc * val,
            '/' => acc / val,
            '+' => acc + val,
            '-' => acc - val,
            _ => {
                panic!("Invalid val");
            }
        },
    )(i)
}

fn expr(i: &str) -> IResult<&str, i64> {
    term(i)
}

fn term_v2(i: &str) -> IResult<&str, i64> {
    let (i, init) = factor_v2(i)?;

    fold_many0(
        pair(alt((char('+'), char('-'))), factor_v2),
        init,
        |acc, (op, val): (char, i64)| match op {
            '+' => acc + val,
            '-' => acc - val,
            _ => {
                panic!("Invalid val");
            }
        },
    )(i)
}

fn expr_v2(i: &str) -> IResult<&str, i64> {
    let (i, init) = term_v2(i)?;

    fold_many0(
        pair(alt((char('*'), char('/'))), term_v2),
        init,
        |acc, (op, val): (char, i64)| {
            if op == '*' {
                acc * val
            } else {
                acc / val
            }
        },
    )(i)
}

fn main() {
    let lines = read_lines("src/inputs/day_18.txt")
        .unwrap()
        .flatten()
        .collect::<Vec<_>>();

    let result = lines
        .iter()
        .map(|line| expr(&line[..]).unwrap().1)
        .sum::<i64>();

    println!("Part 1: {:?}", result);

    let result = lines
        .iter()
        .map(|line| expr_v2(&line[..]).unwrap().1)
        .sum::<i64>();

    println!("Part 2: {:?}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn factor_test() {
        assert_eq!(factor("3"), Ok(("", 3)));
        assert_eq!(factor(" 12"), Ok(("", 12)));
        assert_eq!(factor("537  "), Ok(("", 537)));
        assert_eq!(factor("  24   "), Ok(("", 24)));
    }

    #[test]
    fn term_test() {
        assert_eq!(term(" 12 *2 /  3"), Ok(("", 8)));
        assert_eq!(term(" 2* 3  *2 *2 /  3"), Ok(("", 8)));
        assert_eq!(term(" 48 /  3/2"), Ok(("", 8)));
    }

    #[test]
    fn expr_test() {
        assert_eq!(expr(" 1 +  2 "), Ok(("", 3)));
        assert_eq!(expr(" 12 + 6 - 4+  3"), Ok(("", 17)));
        assert_eq!(expr(" 1 + 2 * 3 + 4"), Ok(("", 13)));
    }

    #[test]
    fn parens_test() {
        assert_eq!(expr(" (  2 )"), Ok(("", 2)));
        assert_eq!(expr(" 2* (  3 + 4 ) "), Ok(("", 14)));
        assert_eq!(expr("  2*2 / ( 5 - 1) + 3"), Ok(("", 4)));
    }

    #[test]
    fn example_p1() {
        assert_eq!(expr("2 * 3 + (4 * 5)"), Ok(("", 26)));
        assert_eq!(expr("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(("", 437)));
        assert_eq!(
            expr("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(("", 12240))
        );
        assert_eq!(
            expr("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 13632))
        );
    }

    #[test]
    fn example_p2() {
        assert_eq!(expr_v2("1 + 2 * 3 + 4 * 5 + 6"), Ok(("", 231)));
        assert_eq!(expr_v2("5 + (8 * 3 + 9 * 4 * 3)"), Ok(("", 1157)));
        assert_eq!(expr_v2("1 + (2 * 3) + (4 * (5 + 6))"), Ok(("", 51)));
        assert_eq!(expr_v2("2 * 3 + (4 * 5)"), Ok(("", 46)));
        assert_eq!(expr_v2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), Ok(("", 1445)));
        assert_eq!(
            expr_v2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            Ok(("", 669060))
        );
        assert_eq!(
            expr_v2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            Ok(("", 23340))
        );
    }
}
