use std::cmp::PartialEq;
use std::collections::HashSet;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let input = Input::from_str(input);
    let visited = simulate_guard_movement(&input.map, input.guard_initial_position);
    match visited {
        MovementSimulationResult::Loop => panic!("part one should not loop!"),
        MovementSimulationResult::Completed(visited) => {
            Some(visited.len() as u32)
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Input::from_str(input);

    // get all the positions we could put an obstacle
    let obstacleable_positions: Vec<(usize, usize)> = input
        .map
        .iter()
        .enumerate()
        .flat_map(|(y_pos, row)| {
            row.iter().enumerate().filter_map(move |(x_pos, slot)| {
                if *slot == MapSlot::Empty && (y_pos, x_pos) != input.guard_initial_position {
                    Some((y_pos, x_pos))
                } else {
                    None
                }
            })
        })
        .collect();

    let count_of_loops = obstacleable_positions
        .par_iter()
        .filter(|(y_pos, x_pos)| {
            let mut new_map = input.map.clone();
            new_map[*y_pos][*x_pos] = MapSlot::Obstacle;

            match simulate_guard_movement(&new_map, input.guard_initial_position) {
                MovementSimulationResult::Loop => true,
                _ => false
            }
        })
        .count();

    Some(count_of_loops as u32)
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum MapSlot {
    Empty,
    Obstacle,
    Guard(Orientation),
}

#[derive(Debug, PartialEq, Copy, Clone, Hash, Eq)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Input {
    map: Vec<Vec<MapSlot>>,
    guard_initial_position: (usize, usize),
}

impl Input {
    fn from_str(input: &str) -> Input {
        fn parse_char_into_mapslot(char: char) -> MapSlot {
            match char {
                '.' => MapSlot::Empty,
                '#' => MapSlot::Obstacle,
                '>' => MapSlot::Guard(Orientation::Right),
                '<' => MapSlot::Guard(Orientation::Left),
                '^' => MapSlot::Guard(Orientation::Up),
                'v' => MapSlot::Guard(Orientation::Down),
                _ => panic!("Unknown char: {}", char)
            }
        }

        let mut guard_pos = None;

        let map = input.lines().enumerate()
            .map(|(ypos, line)| {
                line.chars()
                    .enumerate()
                    .map(|(xpos, char)| {
                        let map_slot = parse_char_into_mapslot(char);
                        if let MapSlot::Guard(_) = &map_slot {
                            if let Some(_) = guard_pos.replace((ypos, xpos)) {
                                panic!("Multiple guards")
                            }
                        }
                        map_slot
                    })
                    .collect::<Vec<MapSlot>>()
            })
            .collect::<Vec<Vec<MapSlot>>>();

        let guard_initial_position = guard_pos.expect("No guard initial position");

        Input {
            map,
            guard_initial_position,
        }
    }
}

#[derive(Debug, PartialEq)]
enum MovementSimulationResult {
    Loop,
    Completed(HashSet<(usize, usize)>),
}

//returns all the positions the guard would visit
fn simulate_guard_movement(map: &Vec<Vec<MapSlot>>, guard_initial_position: (usize, usize)) -> MovementSimulationResult {
    // if against obstacle in orientation, return existing position and new orientation
    // if we would go out of bounds, return None
    // otherwise return new position with existing orientation
    fn move_guard_or_complete(map: &Vec<Vec<MapSlot>>, orientation: Orientation, position: (usize, usize)) -> Option<(Orientation, (usize, usize))> {
        match orientation {
            Orientation::Up => {
                //check if above is out of bounds
                if position.0 == 0 {
                    //we would move out of bounds, return None, we're done
                    return None;
                }
                let new_y_pos = position.0 - 1;

                //check if obstacle is above
                if let MapSlot::Obstacle = &map[new_y_pos][position.1] {
                    // return shifted 90 degrees
                    return Some((Orientation::Right, position));
                }
                //we are clear to move
                Some((orientation, (new_y_pos, position.1)))
            }
            Orientation::Down => {
                let new_y_pos = position.0 + 1;

                //check if down is out of bounds
                if new_y_pos > map.len() - 1 {
                    //we would move out of bounds, return None, we're done
                    return None;
                }
                //check if obstacle is below
                if let MapSlot::Obstacle = &map[new_y_pos][position.1] {
                    // return shifted 90 degrees
                    return Some((Orientation::Left, position));
                }
                //we are clear to move
                Some((orientation, (new_y_pos, position.1)))
            }
            Orientation::Left => {
                //check if left is out of bounds
                if position.1 == 0 {
                    //we would move out of bounds, return None, we're done
                    return None;
                }
                let new_x_pos = position.1 - 1;

                //check if obstacle is left
                if let MapSlot::Obstacle = &map[position.0][new_x_pos] {
                    // return shifted 90 degrees
                    return Some((Orientation::Up, position));
                }
                //we are clear to move
                Some((orientation, (position.0, new_x_pos)))
            }
            Orientation::Right => {
                let new_x_pos = position.1 + 1;
                //check if right is out of bounds
                if new_x_pos > map[position.0].len() - 1 {
                    //we would move out of bounds, return None, we're done
                    return None;
                }
                //check if obstacle is right
                if let MapSlot::Obstacle = &map[position.0][new_x_pos] {
                    // return shifted 90 degrees
                    return Some((Orientation::Down, position));
                }
                //we are clear to move
                Some((orientation, (position.0, new_x_pos)))
            }
        }
    }

    let mut current_guard_pos = guard_initial_position;
    let mut current_orientation = match map[current_guard_pos.0][current_guard_pos.1] {
        MapSlot::Guard(o) => o,
        mapslot => panic!("expected guard at position {:?} but found {:?}", current_guard_pos, mapslot)
    };

    let mut distinct_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut distinct_positions_and_orientations: HashSet<(Orientation, usize, usize)> = HashSet::new();
    //make sure to insert first position
    distinct_positions.insert(current_guard_pos);
    distinct_positions_and_orientations.insert((current_orientation, current_guard_pos.0, current_guard_pos.1));

    while let Some((new_orientation, new_pos)) = move_guard_or_complete(map,
                                                                        current_orientation, current_guard_pos) {
        let inserted = distinct_positions_and_orientations.insert((new_orientation, new_pos.0, new_pos.1));
        if !inserted {
            //we found a loop
            return MovementSimulationResult::Loop;
        }

        distinct_positions.insert((new_pos));
        current_guard_pos = new_pos;
        current_orientation = new_orientation;
    }

    MovementSimulationResult::Completed(distinct_positions)
}


#[cfg(test)]
mod tests {
    use std::f32::consts::E;
    use nom::Or;
    use super::*;

    use MapSlot::*;

    const SMALL_VALID_WITH_GUARD_UP: &str = ".^.#...\n#..#...\n##...#.\n";
    const SMALL_VALID_WITH_GUARD_DOWN: &str = ".v.#...\n#..#...\n##...#.\n";
    const SMALL_VALID_WITH_GUARD_LEFT: &str = ".<.#...\n#..#...\n##...#.\n";
    const SMALL_VALID_WITH_GUARD_RIGHT: &str = ".>.#...\n#..#...\n##...#.\n";

    const SMALL_VALID_WITH_NO_GUARD: &str = "...#...\n#..#...\n##...#.\n";

    const INVALID_MULTIPLE_GUARDS: &str = "..#\n>..\n<..\n";
    const INVALID_UNKNOWN_CHARS: &str = "#..\n5sg\n..^\n";

    #[test]
    fn test_parse_input_small_valid_guard_up() {
        let expected_map = vec![
            vec![Empty, Guard(Orientation::Up), Empty, Obstacle, Empty, Empty, Empty],
            vec![Obstacle, Empty, Empty, Obstacle, Empty, Empty, Empty],
            vec![Obstacle, Obstacle, Empty, Empty, Empty, Obstacle, Empty],
        ];
        let expected = Input {
            map: expected_map,
            guard_initial_position: (0, 1),
        };
        let actual = Input::from_str(SMALL_VALID_WITH_GUARD_UP);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_input_small_valid_guard_down() {
        let expected_map = vec![
            vec![Empty, Guard(Orientation::Down), Empty, Obstacle, Empty, Empty, Empty],
            vec![Obstacle, Empty, Empty, Obstacle, Empty, Empty, Empty],
            vec![Obstacle, Obstacle, Empty, Empty, Empty, Obstacle, Empty],
        ];
        let expected = Input {
            map: expected_map,
            guard_initial_position: (0, 1),
        };
        let actual = Input::from_str(SMALL_VALID_WITH_GUARD_DOWN);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_input_small_valid_guard_left() {
        let expected_map = vec![
            vec![Empty, Guard(Orientation::Left), Empty, Obstacle, Empty, Empty, Empty],
            vec![Obstacle, Empty, Empty, Obstacle, Empty, Empty, Empty],
            vec![Obstacle, Obstacle, Empty, Empty, Empty, Obstacle, Empty],
        ];
        let expected = Input {
            map: expected_map,
            guard_initial_position: (0, 1),
        };
        let actual = Input::from_str(SMALL_VALID_WITH_GUARD_LEFT);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_input_small_valid_guard_right() {
        let expected_map = vec![
            vec![Empty, Guard(Orientation::Right), Empty, Obstacle, Empty, Empty, Empty],
            vec![Obstacle, Empty, Empty, Obstacle, Empty, Empty, Empty],
            vec![Obstacle, Obstacle, Empty, Empty, Empty, Obstacle, Empty],
        ];
        let expected = Input {
            map: expected_map,
            guard_initial_position: (0, 1),
        };
        let actual = Input::from_str(SMALL_VALID_WITH_GUARD_RIGHT);
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn test_parse_input_empty() {
        Input::from_str("");
    }

    #[test]
    #[should_panic]
    fn test_parse_input_multiple_guards() {
        Input::from_str(INVALID_MULTIPLE_GUARDS);
    }


    #[test]
    #[should_panic]
    fn test_parse_input_invalid_chars() {
        Input::from_str(INVALID_UNKNOWN_CHARS);
    }

    #[test]
    fn test_simple_guard_simulation() {
        let first_row = vec![Empty, Guard(Orientation::Down), Empty, Empty, Empty];
        let second_row = vec![Empty, Empty, Empty, Empty, Empty];
        let third_row = vec![Empty, Obstacle, Empty, Obstacle, Empty];
        let map = vec![first_row, second_row, third_row];

        let mut visited = HashSet::new();
        visited.insert((0, 1));
        visited.insert((1, 0));
        visited.insert((1, 1));

        let expected = MovementSimulationResult::Completed(visited);

        let actual = simulate_guard_movement(&map, (0, 1));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_simple_loop() {
        let first_row = vec![Empty, Obstacle, Empty, Empty, Empty];
        let second_row = vec![Empty, Empty, Empty, Guard(Orientation::Down), Obstacle];
        let third_row = vec![Empty, Empty, Empty, Empty, Empty];
        let fourth_row = vec![Obstacle, Empty, Empty, Empty, Empty];
        let fifth_row = vec![Empty, Empty, Empty, Obstacle, Empty];
        let map = vec![first_row, second_row, third_row, fourth_row, fifth_row];

        let expected = MovementSimulationResult::Loop;
        let actual = simulate_guard_movement(&map, (1, 3));
        assert_eq!(actual, MovementSimulationResult::Loop);
    }
}
