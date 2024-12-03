extern crate core;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::{map, map_res};
use nom::sequence::{delimited, tuple};
use nom::{IResult, Parser};

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let input = Input::parse_from_str(input);
    let summed = input.ops.iter().fold(0, |acc, op| {
        match op {
            Operation::Mul(l, r) => acc + (l * r),
            _ => acc, //ignore others
        }
    });

    Some(summed)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Input::parse_from_str(input);
    let result = input
        .ops
        .iter()
        .fold((0, true), |(sum, enabled), op| match op {
            Operation::Do => (sum, true),
            Operation::Dont => (sum, false),
            Operation::Mul(l, r) => match enabled {
                true => (sum + (l * r), enabled),
                false => (sum, enabled),
            },
        });

    Some(result.0)
}

#[derive(Debug, PartialEq)]
pub struct Input {
    ops: Vec<Operation>,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Do,
    Dont,
    Mul(u32, u32),
}

impl Input {
    pub fn parse_from_str(input: &str) -> Self {
        let mut ops = vec![];
        let mut remaining = input;

        while !remaining.is_empty() {
            match alt((
                Self::parse_do_operation,
                Self::parse_dont_operation,
                Self::parse_multiply_operation,
            ))(remaining)
            {
                Ok((remaining_input, op)) => {
                    ops.push(op);
                    remaining = remaining_input;
                }
                Err(_) => {
                    remaining = &remaining[1..];
                }
            }
        }

        Input { ops }
    }

    fn parse_do_operation(input: &str) -> IResult<&str, Operation> {
        map(tag("do()"), |_| Operation::Do)(input)
    }

    fn parse_dont_operation(input: &str) -> IResult<&str, Operation> {
        map(tag("don't()"), |_| Operation::Dont)(input)
    }

    fn parse_multiply_operation(input: &str) -> IResult<&str, Operation> {
        let (input, _) = tag("mul")(input)?;

        let (input, (left, _, right)) = delimited(
            tag("("),
            tuple((
                map_res(digit1, |s: &str| s.parse::<u32>()),
                tag(","),
                map_res(digit1, |s: &str| s.parse::<u32>()),
            )),
            tag(")"),
        )(input)?;

        Ok((input, Operation::Mul(left, right)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_INPUT: &str = "";
    const GIBBERISH_INPUT: &str = "m(erou,43t,65)rem)";

    const SIMPLE_MUL_INPUT: &str = "mul(2,4)";
    const MORE_SIMPLE_MUL_INPUT: &str = "mul(63,23),mul(1,542)";
    const MIXED_MUL_INPUT: &str = "efumul(3,56),25t,mul(5,7)))mul(3,,,)wea32";

    const SIMPLE_DO_INPUT: &str = "do()";
    const SIMPLE_DONT_INPUT: &str = "don't()";
    const SIMPLE_DO_DONT_INPUT: &str = "do()don't()don't()do()";
    const MIXED_DO_DONT_INPUT: &str = "do()oigiybwer3435,ourdo()lfounf,33{{don't(),,,,don't()";
    const FULLY_MIXED_INPUT: &str =
        "gnoudo()en302mul(2,56)49gmul(((no3do(((don't()48mul(32,2156)hg0934g";

    #[test]
    fn test_parse_empty_input() {
        let input = Input::parse_from_str(EMPTY_INPUT);
        assert_eq!(input.ops, vec![]);
    }

    #[test]
    fn test_parse_gibberish_input() {
        let input = Input::parse_from_str(GIBBERISH_INPUT);
        assert_eq!(input.ops, vec![]);
    }

    #[test]
    fn test_parse_simple_mul_input() {
        let input = Input::parse_from_str(SIMPLE_MUL_INPUT);
        assert_eq!(input.ops, vec![Operation::Mul(2, 4)]);
    }

    #[test]
    fn test_parse_more_simple_mul_input() {
        let input = Input::parse_from_str(MORE_SIMPLE_MUL_INPUT);
        assert_eq!(
            input.ops,
            vec![Operation::Mul(63, 23), Operation::Mul(1, 542)]
        );
    }

    #[test]
    fn test_parse_mixed_mul_input() {
        let input = Input::parse_from_str(MIXED_MUL_INPUT);
        assert_eq!(input.ops, vec![Operation::Mul(3, 56), Operation::Mul(5, 7)]);
    }

    #[test]
    fn test_parse_simple_do_input() {
        let input = Input::parse_from_str(SIMPLE_DO_INPUT);
        assert_eq!(input.ops, vec![Operation::Do]);
    }

    #[test]
    fn test_parse_simple_dont_input() {
        let input = Input::parse_from_str(SIMPLE_DONT_INPUT);
        assert_eq!(input.ops, vec![Operation::Dont]);
    }

    #[test]
    fn test_parse_simple_don_dont_input() {
        let input = Input::parse_from_str(SIMPLE_DO_DONT_INPUT);
        assert_eq!(
            input.ops,
            vec![
                Operation::Do,
                Operation::Dont,
                Operation::Dont,
                Operation::Do
            ]
        );
    }

    #[test]
    fn test_parse_mixed_do_dont_input() {
        let input = Input::parse_from_str(MIXED_DO_DONT_INPUT);
        assert_eq!(
            input.ops,
            vec![
                Operation::Do,
                Operation::Do,
                Operation::Dont,
                Operation::Dont
            ]
        );
    }

    #[test]
    fn test_parse_fully_mixed_input() {
        let input = Input::parse_from_str(FULLY_MIXED_INPUT);
        assert_eq!(
            input.ops,
            vec![
                Operation::Do,
                Operation::Mul(2, 56),
                Operation::Dont,
                Operation::Mul(32, 2156)
            ]
        );
    }

    #[test]
    fn test_part_one_mixed_mul_input() {
        let result = part_one(MIXED_MUL_INPUT);
        let expected_result = 203; //calculated manually
        assert_eq!(result, Some(expected_result));
    }

    #[test]
    fn test_part_two_fully_mixed_input() {
        let result = part_one(FULLY_MIXED_INPUT);
        let expected_result = 69104; //calculated manually
        assert_eq!(result, Some(expected_result));
    }
}
