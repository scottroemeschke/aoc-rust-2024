use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::multi::{separated_list0};
use nom::sequence::{separated_pair};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let input = Input::from_str(input);
    let solvable_calibrations = input.calibrations.par_iter()
        .filter(|calibration| can_operators_solve(calibration, false))
        .collect::<Vec<_>>();

    let total_calibration_result = solvable_calibrations.iter().map(|c| c.total).sum();
    Some(total_calibration_result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = Input::from_str(input);
    let solvable_calibrations = input.calibrations.par_iter()
        .filter(|calibration| can_operators_solve(calibration, true))
        .collect::<Vec<_>>();

    let total_calibration_result = solvable_calibrations.iter().map(|c| c.total).sum();
    Some(total_calibration_result)
}

#[derive(Debug, PartialEq)]
struct Input {
    calibrations: Vec<Calibration>,
}

#[derive(Debug, PartialEq)]
struct Calibration {
    total: usize,
    nums: Vec<usize>,
}

impl Calibration {
    fn from_str(input: &str) -> Self {
        let (_, (total_str, nums_as_strs)) = separated_pair(digit1::<&str, nom::error::Error<&str>>, tag(": "), separated_list0(space1, digit1))(input)
            .expect(&format!("Invalid calibration syntax: {}", input));

        let total: usize = total_str.parse().expect(
            &format!("total number failed to parse to u32: {}, on line content: {}", total_str, input)
        );
        let nums: Vec<usize> = nums_as_strs.iter()
            .map(|num| num.parse()
                .expect(&format!("number failed to parse to u32: {}, on line content: {}", num, input)))
            .collect();

        Calibration {
            total,
            nums,
        }
    }
}

impl Input {
    fn from_str(input: &str) -> Self {
        let calibrations = input.lines()
            .map(|line| Calibration::from_str(line))
            .collect::<Vec<_>>();
        Input { calibrations }
    }
}

fn can_operators_solve(calibration: &Calibration, concat_operator_enabled: bool) -> bool {
    let last_nums_index = calibration.nums.len() - 1;

    let mut operations: Vec<fn(usize, usize) -> usize> = vec![
        |lhs, rhs| lhs.checked_add(rhs).expect("overflow on adding"),
        |lhs, rhs| lhs.checked_mul(rhs).expect("overflow on multiplying"),
    ];

    if concat_operator_enabled {
        operations.push(|lhs, rhs| {
            let rhs_str = rhs.to_string();
            let lhs_str = lhs.to_string();
            format!("{}{}", lhs_str, rhs_str).parse().unwrap()
        })
    }

    let num_operation_slots = last_nums_index;

    let ops_orders = std::iter::repeat(&operations)
        .take(num_operation_slots)
        .multi_cartesian_product();

    for op_order in ops_orders {
        // new order of operators, lets try it
        let mut numbers = calibration.nums.iter();
        let mut result = *numbers.next().expect("no initial number");

        for op in op_order {
            let next_num = *numbers.next().expect("too many operators for the amount of numbers");
            result = op(result, next_num);
        }

        if result == calibration.total {
            // found a match, we only need to find one so we are done and can say true
            return true;
        }
    }
    //went through all options, not possible to make it work
    false
}


#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_CALIBRATION: &str = "190: 10 19";
    const LONG_CALIBRATION: &str = "21037: 9 7 18 13";

    const SIMPLE_INPUT: &str = "19: 1 3 5\n223: 5 6 809\n";

    #[test]
    fn test_parse_simple_calibration() {
        let expected = Calibration {
            total: 190,
            nums: vec![10, 19],
        };
        let actual = Calibration::from_str(SIMPLE_CALIBRATION);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_long_calibration() {
        let expected = Calibration {
            total: 21037,
            nums: vec![9, 7, 18, 13],
        };
        let actual = Calibration::from_str(LONG_CALIBRATION);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_simple_input() {
        let expected = Input {
            calibrations: vec![
                Calibration {
                    total: 19,
                    nums: vec![1, 3, 5]
                },
                Calibration {
                    total: 223,
                    nums: vec![5, 6, 809]
                }
            ]
        };
        let actual = Input::from_str(SIMPLE_INPUT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_calibration_can_be_solved_simple_addition() {
        let calibration = Calibration {
            total: 25,
            nums: vec![5, 19, 1],
        };
        let actual = can_operators_solve(&calibration);
        assert_eq!(true, actual);
    }

    #[test]
    fn test_calibration_can_be_solved_simple_multiplication() {
        let calibration = Calibration {
            total: 190,
            nums: vec![10, 19],
        };
        let actual = can_operators_solve(&calibration);
        assert_eq!(true, actual);
    }

    #[test]
    fn test_calibration_can_be_solved_mixed_operations() {
        let calibration = Calibration {
            total: 200,
            nums: vec![8, 2, 20],
        };
        let actual = can_operators_solve(&calibration);
        assert_eq!(true, actual);
    }

    #[test]
    fn test_calibration_cant_be_solved() {
        let calibration = Calibration {
            total: 2521,
            nums: vec![5, 2, 1, 1, 5],
        };
        let actual = can_operators_solve(&calibration);
        assert_eq!(false, actual);
    }
}
