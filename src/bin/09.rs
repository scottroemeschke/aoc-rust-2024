advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let input = Input::from_str(input);
    let defragged = defrag_bitwise(&input.blocks);
    let checksum = calculate_checksum(&defragged);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = Input::from_str(input);
    let defragged = defrag_filewise(&input.blocks, &input.repr);
    let checksum = calculate_checksum(&defragged);
    Some(checksum)
}

#[derive(Debug, PartialEq, Clone)]
enum DiskBlock {
    Empty,
    File(u64),
}

#[derive(Debug, PartialEq)]
struct Input {
    repr: Vec<u8>,
    blocks: Vec<DiskBlock>,
}

impl Input {
    fn from_str(str: &str) -> Self {
        let mut chars = str.chars();
        let mut blocks = Vec::new();
        let mut repr = Vec::new();
        let mut current_file_id = 0;

        //expects that we end with free space, and that input is valid with pairs
        while let (Some(file_size_amount), Some(free_space_amount)) = (chars.next(), chars.next()) {
            let file_amount = file_size_amount.to_digit(10).unwrap() as u8;
            repr.push(file_amount);
            for _ in 0..file_amount {
                blocks.push(DiskBlock::File(current_file_id));
            }

            let space_amount = free_space_amount.to_digit(10);

            //handle end
            if let Some(space) = space_amount {
                repr.push(space as u8);
                for _ in 0..space {
                    blocks.push(DiskBlock::Empty);
                }
            }

            if file_amount > 0 {
                current_file_id += 1;
            }
        }

        Input {
            repr,
            blocks,
        }
    }
}

fn calculate_checksum(blocks: &Vec<DiskBlock>) -> u64 {
    blocks
        .iter()
        .enumerate()
        .map(|(position, b)| {
            match b {
                DiskBlock::File(id) => (position as u64) * id,
                DiskBlock::Empty => 0
            }
        })
        .sum()
}

fn defrag_bitwise(blocks: &Vec<DiskBlock>) -> Vec<DiskBlock> {
    let mut defragged = blocks.clone();

    // Find the first empty position
    let mut empty_pos = 0;
    while empty_pos < defragged.len() && defragged[empty_pos] != DiskBlock::Empty {
        empty_pos += 1;
    }

    // Iterate through the list in reverse
    for file_pos in (0..defragged.len()).rev() {
        // Stop if we've gone past the empty position
        if file_pos < empty_pos {
            break;
        }

        // Only move if current position is a file
        if let DiskBlock::File(_) = defragged[file_pos] {
            // Swap the file with the empty position
            defragged.swap(empty_pos, file_pos);

            // Find the next empty position
            empty_pos = (empty_pos + 1..defragged.len())
                .find(|&i| defragged[i] == DiskBlock::Empty)
                .unwrap_or(defragged.len());
        }
    }

    defragged
}

fn defrag_filewise(blocks: &Vec<DiskBlock>, repr: &Vec<u8>) -> Vec<DiskBlock> {
    let mut defragged = blocks.clone();

    //find the next backwards file
    for initial_position in (0..repr.len()).rev().step_by(2) {
        let file_size = repr.get(initial_position).expect("out of bounds position in repr");
        //trying to move file

        //lets find its index in the actual blocks expanded version
        let mut file_block_start = 0;

        for pair_index in (0..initial_position).step_by(2) {
            let fsize = repr[pair_index] as usize;
            let fspace = if pair_index + 1 < repr.len() { repr[pair_index + 1] as usize } else { 0 };
            file_block_start += fsize + fspace;
        }

        let mut start_pos_to_swap = None;

        //find next empty section that would fit that file size
        let mut in_empty = false;
        let mut current_empty_amount = 0;
        let mut current_empty_start_position = 0;
        for (search_pos, search_block) in defragged.iter().enumerate() {
            if search_pos >= file_block_start {
                //we are at or past the index where the file is. we aren't gonna move it back, lets move on to the next file
                break;
            }
            match in_empty {
                false => {
                    if let DiskBlock::Empty = search_block {
                        current_empty_start_position = search_pos;
                        current_empty_amount += 1;
                        in_empty = true;
                    } else {
                        in_empty = false;
                    }
                }
                true => {
                    if let DiskBlock::Empty = search_block {
                        current_empty_amount += 1;
                    } else {
                        //we were in empty, but now we hit a file and previous empty amount was not enough to move
                        current_empty_amount = 0;
                        in_empty = false;
                    }
                }
            }
            if in_empty && current_empty_amount == (*file_size as usize) {
                //found enough free space to move a whole file!
                start_pos_to_swap = Some(current_empty_start_position);
                break;
            }
        }

        if let Some(pos) = start_pos_to_swap {
            for i in 0..*file_size {
                defragged.swap(pos + (i as usize), file_block_start + (i as usize));
            }
        }
    }
    defragged
}

#[cfg(test)]
mod tests {
    use super::*;
    use DiskBlock::*;

    const SIMPLE_INPUT: &str = "202056";

    /*
    tests were helpful then changed input format, will maybe fix later
    #[test]
    fn test_parse_simple_input() {
        let expected = Input {
            blocks: vec![File(0), File(0), File(1), File(1),
                         File(2), File(2), File(2), File(2),
                         File(2), Empty, Empty, Empty,
                         Empty, Empty, Empty],
        };
        let actual = Input::from_str(SIMPLE_INPUT);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_defrag_simple_input() {
        let blocks = vec![File(0), File(0), File(1), File(1),
                          File(2), File(2), File(2), File(2),
                          File(2), Empty, Empty, Empty,
                          Empty, Empty, Empty];
        let actual = defrag_bitwise(&blocks);

        let expected_checksum = 65;
        let expected_blocks = vec![File(0), File(0), File(1), File(1), File(2), File(2), File(2), File(2), File(2), Empty, Empty, Empty, Empty, Empty, Empty];
        assert_eq!(actual, (expected_blocks, expected_checksum));
    }

     */
}
