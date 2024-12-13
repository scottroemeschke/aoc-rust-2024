use std::collections::HashSet;
use dashmap::DashMap;
use pathfinding::prelude::bfs;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let input = Input::from_str(input);
    let trailheads = get_all_trailhead_positions(&input.map);
    let total_score = trailheads.par_iter()
        .map(|th| {
            calculate_score_for_trailhead_position(&input.map, th)
        })
        .sum();
    Some(total_score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = Input::from_str(input);
    let trailheads = get_all_trailhead_positions(&input.map);
    let memoized_trailcount_from_each_position = DashMap::new();
    let total_rating = trailheads.par_iter().map(|th| {
        calculate_rating_for_trailhead(&input.map, th, &memoized_trailcount_from_each_position)
    }).sum();
    Some(total_rating)
}

struct Input {
    map: Vec<Vec<u8>>,
}

impl Input {
    fn from_str(input: &str) -> Self {
        let map = input.lines()
            .map(|line| {
                line
                    .chars()
                    .map(|ch| (ch.to_digit(10).unwrap() as u8))
                    .collect()
            })
            .collect();
        Input {
            map
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct TrailheadPosition {
    x: usize,
    y: usize,
}

const DIRECTIONS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
const TARGET_HEIGHT: u8 = 9u8;

fn calculate_rating_for_trailhead(
    map: &Vec<Vec<u8>>,
    tp: &TrailheadPosition,
    memo: &DashMap<(usize, usize), usize>,
) -> usize {
    fn count_trails(
        map: &Vec<Vec<u8>>,
        y: usize,
        x: usize,
        memo: &DashMap<(usize, usize), usize>,
    ) -> usize {
        if let Some(already_calculated) = memo.get(&(y, x)) {
            return *already_calculated;
        }

        let current_height = map[y][x];
        if current_height == TARGET_HEIGHT {
            //all there is
            return 1;
        }

        let mut total_trails = 0;

        for &(dx, dy) in &DIRECTIONS {
            let nx = x as isize + dx as isize;
            let ny = y as isize + dy as isize;

            if nx >= 0
                && ny >= 0
                && (ny as usize) < map.len()
                && (nx as usize) < map[0].len()
            {
                let nx = nx as usize;
                let ny = ny as usize;

                if map[ny][nx] == current_height + 1 {
                    total_trails += count_trails(map, ny, nx, memo);
                }
            }
        }

        //insert so we can reuse this calculation again
        memo.insert((y, x), total_trails);
        total_trails
    }

    count_trails(map, tp.y, tp.x, memo)
}

fn get_all_trailhead_positions(map: &Vec<Vec<u8>>) -> Vec<TrailheadPosition> {
    let mut positions = Vec::with_capacity(850);
    for (y, row) in map.iter().enumerate() {
        for (x, num) in row.iter().enumerate() {
            if *num == 0 {
                positions.push(TrailheadPosition { x, y });
            }
        }
    }
    positions
}

fn calculate_score_for_trailhead_position(map: &Vec<Vec<u8>>, tp: &TrailheadPosition) -> usize {
    fn traversable_neighbors_of_position(map: &Vec<Vec<u8>>, y: usize, x: usize) -> Vec<(usize, usize)> {
        let mut result = Vec::with_capacity(4);

        for &(dx, dy) in &DIRECTIONS {
            let nx = x as isize + dx as isize;
            let ny = y as isize + dy as isize;

            if nx >= 0
                && ny >= 0
                && (ny as usize) < map.len()
                && (nx as usize) < map[0].len()
            {
                let nx = nx as usize;
                let ny = ny as usize;

                //only count heights that are exactly 1 up
                if map[ny][nx] == map[y][x] + 1 {
                    result.push((ny, nx));
                }
            }
        }

        result
    }

    //use set for auto de-duping of positions, we should only count each 9 once, regardless of multiple paths to get there
    let mut nines_we_can_reach = HashSet::new();

    bfs(
        &(tp.y, tp.x),
        |&(y, x)| traversable_neighbors_of_position(map, y, x),
        |&(y, x)| {
            if map[y][x] == TARGET_HEIGHT {
                nines_we_can_reach.insert((y, x));
            }
            //keep going, it's not a "first success ends the search"
            false
        },
    );

    //println!("final score for trailhead position: {:?}, is: {:?}", tp, nines_we_can_reach.len());

    nines_we_can_reach.len()
}


#[cfg(test)]
mod tests {
    use super::*;
}
