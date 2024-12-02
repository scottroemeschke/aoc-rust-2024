use std::collections::hash_map::Entry;
use std::collections::HashMap;

advent_of_code::solution!(1);

struct Input {
    pub(crate) left_column: Vec<u32>,
    pub(crate) right_column: Vec<u32>,
}

impl Input {
    fn from_str(input: &str) -> Self {
        let (mut left_column, mut right_column): (Vec<_>, Vec<_>) = input
            .lines()
            .map(|line| {
                let mut split = line.split_whitespace();
                (
                    split.next().unwrap().parse::<u32>().unwrap(),
                    split.next().unwrap().parse::<u32>().unwrap(),
                )
            })
            .unzip();

        assert_eq!(
            left_column.len(),
            right_column.len(),
            "input columns must be the same length"
        );

        left_column.sort();
        right_column.sort();

        Input {
            left_column,
            right_column,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let inp = Input::from_str(input);
    let total_diff = inp
        .left_column
        .iter()
        .zip(inp.right_column.iter())
        .map(|(lv, rv)| u32::abs_diff(*lv, *rv))
        .sum();

    Some(total_diff)
}

pub fn part_two(input: &str) -> Option<u32> {
    let inp = Input::from_str(input);

    let mut counts_in_rhs = HashMap::new();
    for &num in inp.right_column.iter() {
        match counts_in_rhs.entry(num) {
            Entry::Occupied(mut e) => {
                *e.get_mut() += 1;
            }
            Entry::Vacant(e) => {
                e.insert(1);
            }
        }
    }

    let sim = inp
        .left_column
        .iter()
        .fold(0, |acc, n| acc + (n * counts_in_rhs.get(n).unwrap_or(&0)));

    Some(sim)
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_INPUT_NO_MATCHES_ACROSS_COLUMNS: &str = "3 4\n1 2\n5 6";
    const VALID_INPUT_MATCHES_ACROSS_COLUMNS: &str = "3 4\n1 3\n5 3";
    const INVALID_INPUT_LENGTH: &str = "3 4\n1";
    const INVALID_INPUT_NOT_INT_VALUES: &str = "a b\nc d";
    const INVALID_INPUT_NO_WHITESPACE: &str = "3\n1 2\n5 6";

    #[test]
    fn test_parse_input_valid_input() {
        let parsed = Input::from_str(VALID_INPUT_NO_MATCHES_ACROSS_COLUMNS);
        assert_eq!(parsed.left_column, vec![1, 3, 5]);
        assert_eq!(parsed.right_column, vec![2, 4, 6]);
    }

    #[test]
    #[should_panic]
    fn test_parse_input_invalid_input_length_mismatch() {
        Input::from_str(INVALID_INPUT_LENGTH);
    }

    #[test]
    fn test_parse_input_empty_input() {
        let parsed = Input::from_str("");
        assert_eq!(parsed.left_column, Vec::<u32>::new());
        assert_eq!(parsed.right_column, Vec::<u32>::new());
    }

    #[test]
    #[should_panic]
    fn test_parse_input_non_integer_values() {
        Input::from_str(INVALID_INPUT_NOT_INT_VALUES);
    }

    #[test]
    #[should_panic]
    fn test_parse_input_no_whitespace() {
        Input::from_str(INVALID_INPUT_NO_WHITESPACE);
    }

    #[test]
    fn test_day_1_valid_input() {
        let result = part_one(VALID_INPUT_NO_MATCHES_ACROSS_COLUMNS);
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_day_2_valid_input_matches() {
        let result = part_two(VALID_INPUT_MATCHES_ACROSS_COLUMNS);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_day_2_valid_input_no_matches() {
        let result = part_two(VALID_INPUT_NO_MATCHES_ACROSS_COLUMNS);
        assert_eq!(result, Some(0));
    }
}
