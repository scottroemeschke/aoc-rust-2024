advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let input = Input::from_str(input);
    let matches = input.find_matching_word_in_all_directions("XMAS");
    Some(matches)
}

struct Input {
    grid: Vec<Vec<char>>,
}

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 0),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

impl Input {
    fn from_str(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let grid = lines.iter().map(|line| line.chars().collect()).collect();
        Self { grid }
    }

    fn find_matching_word_in_all_directions(&self, word: &str) -> u32 {
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut count = 0;

        let chars: Vec<char> = word.chars().collect();

        for x in 0..rows {
            for y in 0..cols {
                for (x_direction, y_direction) in &DIRECTIONS {
                    let mut matched = true;
                    for i in 0..chars.len() {
                        let nx = x as isize + i as isize * x_direction;
                        let ny = y as isize + i as isize * y_direction;

                        //if our direction would take us out of bounds, no match
                        if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                            matched = false;
                            break;
                        }

                        //didn't go out of bounds, but it's not a match
                        if self.grid[nx as usize][ny as usize] != chars[i] {
                            matched = false;
                            break;
                        }
                    }
                    if matched {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = Input::from_str(input);

    let rows = input.grid.len();
    let cols = input.grid[0].len();
    let mut count = 0;

    // Helper function to check for a MAS pattern in a given direction
    fn find_matching_mas_in_direction(
        grid: &[Vec<char>],
        start_x: isize,
        start_y: isize,
        dx: isize,
        dy: isize,
    ) -> bool {
        let rows = grid.len() as isize;
        let cols = grid[0].len() as isize;

        let forward_mas = ['M', 'A', 'S'];
        let backward_mas = ['S', 'A', 'M'];

        //prechecking if mas wouldn't go out of bounds
        if start_x >= 0
            && start_x + 2 * dx >= 0
            && start_x + 2 * dx < rows
            && start_y >= 0
            && start_y + 2 * dy >= 0
            && start_y + 2 * dy < cols
        {
            let chars = vec![
                grid[(start_x) as usize][(start_y) as usize],
                grid[(start_x + dx) as usize][(start_y + dy) as usize],
                grid[(start_x + 2 * dx) as usize][(start_y + 2 * dy) as usize],
            ];
            chars == forward_mas || chars == backward_mas
        } else {
            false
        }
    }

    for x in 1..(rows - 1) {
        for y in 1..(cols - 1) {
            if input.grid[x][y] == 'A' {
                //check both directions from the A
                if find_matching_mas_in_direction(&input.grid, x as isize - 1, y as isize - 1, 1, 1)
                    && find_matching_mas_in_direction(
                        &input.grid,
                        x as isize + 1,
                        y as isize - 1,
                        -1,
                        1,
                    )
                {
                    count += 1;
                }
            }
        }
    }

    Some(count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_from_str() {
        let input = "ABC\nDEF\nGHI";
        let input_struct = Input::from_str(input);
        assert_eq!(input_struct.grid.len(), 3);
        assert_eq!(input_struct.grid[0], vec!['A', 'B', 'C']);
        assert_eq!(input_struct.grid[1], vec!['D', 'E', 'F']);
        assert_eq!(input_struct.grid[2], vec!['G', 'H', 'I']);
    }

    #[test]
    fn test_part_one_example_case() {
        let input = "MMMSXXMASM\n\
                     MSAMXMSMSA\n\
                     AMXSXMAAMM\n\
                     MSAMASMSMX\n\
                     XMASAMXAMM\n\
                     XXAMMXXAMA\n\
                     SMSMSASXSS\n\
                     SAXAMASAAA\n\
                     MAMMMXMMMM\n\
                     MXMXAXMASX";
        let result = part_one(input);
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one_puzzle_solution() {
        let input = "MMMSXXMASM\n\
                     MSAMXMSMSA\n\
                     AMXSXMAAMM\n\
                     MSAMASMSMX\n\
                     XMASAMXAMM\n\
                     XXAMMXXAMA\n\
                     SMSMSASXSS\n\
                     SAXAMASAAA\n\
                     MAMMMXMMMM\n\
                     MXMXAXMASX";
        let result = part_one(input);
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two_example_case() {
        let input = ".M.S......\n\
                     ..A..MSMS.\n\
                     .M.S.MAA..\n\
                     ..A.ASMSM.\n\
                     .M.S.M....\n\
                     ..........\n\
                     S.S.S.S.S.\n\
                     .A.A.A.A..\n\
                     M.M.M.M.M.\n\
                     ..........";
        let result = part_two(input);
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_puzzle_solution() {
        let input = ".M.S......\n\
                     ..A..MSMS.\n\
                     .M.S.MAA..\n\
                     ..A.ASMSM.\n\
                     .M.S.M....\n\
                     ..........\n\
                     S.S.S.S.S.\n\
                     .A.A.A.A..\n\
                     M.M.M.M.M.\n\
                     ..........";
        let result = part_two(input);
        assert_eq!(result, Some(9));
    }
}
