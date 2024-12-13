use std::collections::{HashMap, HashSet};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let input = Input::from_str(input);
    let ans = calculate_antinode_coordinates(&input.antenna_locations, input.final_coordinate);
    Some(ans.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Input::from_str(input);
    let ans = calculate_all_antinodes_part_two(&input.antenna_locations, input.final_coordinate);
    Some(ans.len() as u32)
}

#[derive(Debug, PartialEq)]
struct Input {
    antenna_locations: HashMap<char, HashSet<Coordinate>>,
    final_coordinate: Coordinate,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Input {
    fn from_str(input: &str) -> Input {
        let mut locations = HashMap::new();
        for (y_pos, line) in input.lines().enumerate() {
            for (x_pos, char) in line.chars().enumerate() {
                if char == '.' {
                    continue;
                }
                let coord = Coordinate { x: x_pos, y: y_pos };
                locations.entry(char).or_insert(HashSet::new()).insert(coord);
            }
        }

        let final_coordinate = Coordinate {
            x: input.lines().next().map_or(0, |line| line.chars().count() - 1),
            y: input.lines().count() - 1,
        };

        Input {
            antenna_locations: locations,
            final_coordinate,
        }
    }
}

fn calculate_antinode_coordinates(antenna_locations: &HashMap<char, HashSet<Coordinate>>, final_coord: Coordinate) -> HashSet<Coordinate> {
    antenna_locations
        .par_iter()
        .flat_map(|(_, locations)| {
            let positions: Vec<Coordinate> = locations.iter().cloned().collect();

            let mut frequency_antinodes = HashSet::new();

            for i in 1..positions.len() {
                for j in 0..i {
                    if let Some(antinode) = get_antinode(&positions[i], &positions[j], final_coord) {
                        frequency_antinodes.insert(antinode);
                    }

                    if let Some(antinode) = get_antinode(&positions[j], &positions[i], final_coord) {
                        frequency_antinodes.insert(antinode);
                    }
                }
            }

            frequency_antinodes
        })
        .collect()
}

fn get_antinode(source: &Coordinate, other: &Coordinate, final_coord: Coordinate) -> Option<Coordinate> {
    let x_diff = source.x.abs_diff(other.x);

    let x = if source.x < other.x {
        source.x.checked_sub(x_diff)?
    } else {
        source.x + x_diff
    };


    let y_diff = source.y.abs_diff(other.y);

    let y = if source.y < other.y {
        source.y.checked_sub(y_diff)?
    } else {
        source.y + y_diff
    };

    let antinode = Coordinate { x, y };

    // bounds check
    if x <= final_coord.x && y <= final_coord.y {
        Some(antinode)
    } else {
        None
    }
}

fn calculate_all_antinodes_part_two(
    antenna_locations: &HashMap<char, HashSet<Coordinate>>,
    final_coord: Coordinate,
) -> HashSet<Coordinate> {
    antenna_locations
        .par_iter()
        .flat_map(|(_, locations)| {
            let positions: Vec<Coordinate> = locations.iter().cloned().collect();
            let mut frequency_antinodes = HashSet::new();

            frequency_antinodes.extend(&positions);

            for i in 1..positions.len() {
                for j in 0..i {
                    let p1 = positions[i];
                    let p2 = positions[j];

                    let collinear_points = get_spots_on_line(&p1, &p2, final_coord);
                    frequency_antinodes.extend(collinear_points);
                }
            }

            frequency_antinodes
        })
        .collect()
}

fn get_spots_on_line(p1: &Coordinate, p2: &Coordinate, final_coord: Coordinate) -> HashSet<Coordinate> {
    let mut points = HashSet::new();

    let dx = p2.x as isize - p1.x as isize;
    let dy = p2.y as isize - p1.y as isize;

    let gcd = num::integer::gcd(dx, dy);
    let step_x = dx / gcd;
    let step_y = dy / gcd;


    // forward
    let mut current = *p1;
    while current.x <= final_coord.x && current.y <= final_coord.y {
        points.insert(current);

        current = Coordinate {
            x: (current.x as isize + step_x) as usize,
            y: (current.y as isize + step_y) as usize,
        };
    }

    // backward
    current = *p1;
    while current.x <= final_coord.x && current.y <= final_coord.y {
        points.insert(current);

        // Step backward
        current = Coordinate {
            x: (current.x as isize - step_x) as usize,
            y: (current.y as isize - step_y) as usize,
        };
    }

    points
}


#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str = "...\na.7\na.B\n";
    const NO_ANTENNA_INPUT: &str = "...\n...\n...\n";

    #[test]
    fn test_parse_simple_input() {
        let mut locs = HashMap::new();
        let mut a_cords = HashSet::new();
        a_cords.insert(Coordinate { x: 0, y: 1 });
        a_cords.insert(Coordinate { x: 0, y: 2 });
        locs.insert('a', a_cords);

        let mut b_cords = HashSet::new();
        b_cords.insert(Coordinate { x: 2, y: 2 });
        locs.insert('B', b_cords);

        let mut seven_cords = HashSet::new();
        seven_cords.insert(Coordinate { x: 2, y: 1 });
        locs.insert('7', seven_cords);


        let actual = Input::from_str(SIMPLE_INPUT);
        let expected = Input {
            antenna_locations: locs,
            final_coordinate: Coordinate { x: 2, y: 2 },
        };
        assert_eq!(actual, expected);
    }

    #[test]
    #[should_panic]
    fn test_parse_empty_input() {
        let actual = Input::from_str("");
    }

    #[test]
    fn test_parse_no_antenna_input() {
        let actual = Input::from_str(NO_ANTENNA_INPUT);
        let expected = Input {
            antenna_locations: HashMap::new(),
            final_coordinate: Coordinate { x: 2, y: 2 },
        };
        assert_eq!(actual, expected);
    }
}
