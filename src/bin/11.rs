use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::multi::{separated_list1};
use nom::sequence::terminated;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let input = Input::from_str(input);
    let stone_count = blink_large_times_and_get_stone_count(input.stones, 25);
    Some(stone_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = Input::from_str(input);
    let stone_count = blink_large_times_and_get_stone_count(input.stones, 75);
    Some(stone_count)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Stone {
    number: u64,
}

struct Input {
    stones: Vec<Stone>,
}

impl Input {
    fn from_str(input: &str) -> Self {
        let matches = terminated(separated_list1(space1, digit1::<&str, nom::error::Error<&str>>), tag("\n"))
            (input)
            .expect("failed to match on input").1;
        let stones = matches.into_iter().map(|s| {
            let num = s.parse().expect("failed to parse as u64");
            Stone { number: num }
        })
            .collect::<Vec<_>>();
        //println!("final stones from input is: {:?}", stones);
        Input {
            stones
        }
    }
}

fn blink_large_times_and_get_stone_count(starting_stones: Vec<Stone>, blink_count: usize) -> u64 {
    //store how a specific stone updates when blunked
    let mut memoized_effect: HashMap<Stone, Vec<Stone>> = HashMap::new();

    // Store the current counts of each stone type
    let mut current_stones: HashMap<Stone, u64> = HashMap::new();
    for stone in starting_stones {
        *current_stones.entry(stone).or_default() += 1;
    }

    for _ in 0..blink_count {
        let mut next_stones: HashMap<Stone, u64> = HashMap::new();
        for (stone, count) in current_stones {
            if stone.number == 0 {
                *next_stones.entry(Stone { number: 1 }).or_default() += count;
            } else if let Some(resulting_stones) = memoized_effect.get(&stone) {
                //we've already calculated what needs to happen,
                // just update the counts on the new stones, and decrement the count of the old one
                for rs in resulting_stones {
                    *next_stones.entry(*rs).or_default() += count;
                }
            } else {
                //calculate and store the new results in the memoizer
                let number_of_digits = stone.number.ilog10() + 1;
                if number_of_digits % 2 == 0 {
                    let num_str = stone.number.to_string();
                    let middle = num_str.len() / 2;

                    let left = &num_str[..middle];
                    let right = &num_str[middle..];

                    let left_num = left.parse::<u64>().unwrap();
                    let right_num = right.parse::<u64>().unwrap();

                    let left_stone = Stone { number: left_num };
                    let right_stone = Stone { number: right_num };

                    *next_stones.entry(left_stone).or_default() += count;
                    *next_stones.entry(right_stone).or_default() += count;

                    memoized_effect.insert(stone, vec![left_stone, right_stone]);
                } else {
                    let new_stone = Stone { number: stone.number * 2024 };
                    *next_stones.entry(new_stone).or_default() += count;
                    memoized_effect.insert(stone, vec![new_stone]);
                }
            }
        }
        current_stones = next_stones;
    }

    current_stones.values().sum()
}

/*
too slow for part 2, adjusted to just using counts, but leaving here for fun
fn blink(stones: Vec<Stone>) -> Vec<Stone> {
    let original_length = stones.len();
    stones.into_par_iter()
        .map(|s| {
            return if s.number == 0 {
                vec![Stone { number: 1 }]
            } else if s.number.to_string().len() % 2 == 0 {
                let num_str = s.number.to_string();
                let middle = num_str.len() / 2;

                let left = &num_str[..middle];
                let right = &num_str[middle..];

                //println!("split number string: was:{},now left:{},right:{}", num_str, left, right);

                let left_num = left.parse::<u64>().unwrap();
                let right_num = right.parse::<u64>().unwrap();

                vec![Stone { number: left_num }, Stone { number: right_num }]
            } else {
                vec![Stone { number: s.number * 2024 }]
            };
        })
        .reduce(|| Vec::with_capacity(original_length * 2), |mut acc, stones| {
            acc.extend(stones);
            acc
        })
}
 */


#[cfg(test)]
mod tests {
    use super::*;
}
