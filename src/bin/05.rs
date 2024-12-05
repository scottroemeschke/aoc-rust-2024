use rayon::iter::{IntoParallelRefMutIterator, ParallelIterator};
use rayon::prelude::IntoParallelRefIterator;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let input = Input::from_str(input);
    let summed_middle_pages_for_valid_updates = input
        .updates
        .par_iter()
        .filter(|update| update.is_valid_for_rules(&input.before_rules))
        .map(|update| update.get_middle_page())
        .sum::<usize>();
    Some(summed_middle_pages_for_valid_updates as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input = Input::from_str(input);
    let summed_middle_pages_for_invalid_updates_after_fixed = input
        .updates
        .par_iter_mut()
        .filter_map(|update| {
            if update.reorder_with_rules(&input.before_rules) {
                Some(update.get_middle_page())
            } else {
                None
            }
        })
        .sum::<usize>();

    Some(summed_middle_pages_for_invalid_updates_after_fixed as u32)
}

#[derive(Debug, PartialEq)]
struct BeforeRule {
    page_before: usize,
    page_after: usize,
}

#[derive(Debug, PartialEq)]
struct Update {
    pages: Vec<usize>,
}

#[derive(Debug, PartialEq)]
struct Input {
    before_rules: Vec<BeforeRule>,
    updates: Vec<Update>,
}

impl Input {
    fn from_str(input: &str) -> Self {
        let (rules_substr, updates_substr) = input
            .split_once("\n\n")
            .expect("input has more than one empty line");

        //parse tall the rules first
        let rules_lines = rules_substr.lines();
        let before_rules: Vec<BeforeRule> = rules_lines
            .map(|line| {
                let (before, after) = line
                    .split_once("|")
                    .map(|(first, second)| {
                        let f = first
                            .parse::<usize>()
                            .expect("can't parse before to number in rule");
                        let s = second
                            .parse::<usize>()
                            .expect("can't parse after to number in rule");
                        (f, s)
                    })
                    .expect("rule had no | char or multiple | chars");
                BeforeRule {
                    page_before: before,
                    page_after: after,
                }
            })
            .collect();

        //now parse all the updates
        let updates_substr = updates_substr.lines();
        let updates: Vec<Update> = updates_substr
            .map(|line| {
                let pages = line
                    .split(',')
                    .map(|num_str| num_str.parse::<usize>().expect("can't parse update number"))
                    .collect();
                Update { pages }
            })
            .collect();

        Input {
            before_rules,
            updates,
        }
    }
}

impl Update {
    fn is_valid_for_rules(&self, rules: &Vec<BeforeRule>) -> bool {
        for rule in rules {
            let before_pos = self.pages.iter().position(|&x| x == rule.page_before);
            let after_pos = self.pages.iter().position(|&x| x == rule.page_after);

            if let (Some(b_idx), Some(a_idx)) = (before_pos, after_pos) {
                if b_idx > a_idx {
                    return false;
                }
            }
        }
        true
    }

    fn get_middle_page(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }

    //returns whether it was modified or not
    fn reorder_with_rules(&mut self, rules: &[BeforeRule]) -> bool {
        let mut modified = false;

        let mut stabilized = false;
        while !stabilized {
            stabilized = true; // we are considered "stable" until we make a swap
            for rule in rules {
                if let (Some(before_index), Some(after_index)) = (
                    self.pages.iter().position(|&x| x == rule.page_before),
                    self.pages.iter().position(|&x| x == rule.page_after),
                ) {
                    if before_index > after_index {
                        self.pages.swap(before_index, after_index);
                        stabilized = false; //we made a swap, book keep that so we start again
                        modified = true;
                    }
                }
            }
        }

        modified
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE_INPUT: &str =
        "24|64\n24|75\n39|29\n\n18,46,96,13,27,49\n75,58,64,12\n27,18,64,12,65,61,73,98,19,87,53";

    #[test]
    fn test_parse_input_from_str() {
        let expected_rules = vec![
            BeforeRule {
                page_before: 24,
                page_after: 64,
            },
            BeforeRule {
                page_before: 24,
                page_after: 75,
            },
            BeforeRule {
                page_before: 39,
                page_after: 29,
            },
        ];
        let expected_updates = vec![
            Update {
                pages: vec![18, 46, 96, 13, 27, 49],
            },
            Update {
                pages: vec![75, 58, 64, 12],
            },
            Update {
                pages: vec![27, 18, 64, 12, 65, 61, 73, 98, 19, 87, 53],
            },
        ];

        let input = Input::from_str(SIMPLE_INPUT);

        assert_eq!(input.before_rules, expected_rules);
        assert_eq!(input.updates, expected_updates);
    }

    #[test]
    fn test_middle_page() {
        let update = Update {
            pages: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(update.get_middle_page(), 3);
    }

    #[test]
    fn test_update_valid_no_violations() {
        let update = Update {
            pages: vec![1, 2, 3, 4, 5],
        };
        let rules = vec![
            BeforeRule {
                page_before: 1,
                page_after: 2,
            },
            BeforeRule {
                page_before: 2,
                page_after: 3,
            },
            BeforeRule {
                page_before: 3,
                page_after: 4,
            },
            BeforeRule {
                page_before: 4,
                page_after: 5,
            },
        ];

        assert_eq!(update.is_valid_for_rules(&rules), true);
    }

    #[test]
    fn test_update_valid_some_violations() {
        let update = Update {
            pages: vec![1, 2, 3, 4, 5],
        };
        let rules = vec![
            BeforeRule {
                page_before: 2,
                page_after: 1,
            },
            BeforeRule {
                page_before: 2,
                page_after: 3,
            },
            BeforeRule {
                page_before: 4,
                page_after: 5,
            },
            BeforeRule {
                page_before: 20,
                page_after: 50,
            },
            BeforeRule {
                page_before: 45,
                page_after: 6,
            },
        ];

        assert_eq!(update.is_valid_for_rules(&rules), false);
    }

    #[test]
    fn test_update_reorder_no_violations() {
        let seed_pages = vec![1, 2, 3, 4, 5];
        let mut update = Update {
            pages: seed_pages.clone(),
        };
        let rules = vec![
            BeforeRule {
                page_before: 1,
                page_after: 2,
            },
            BeforeRule {
                page_before: 2,
                page_after: 3,
            },
            BeforeRule {
                page_before: 3,
                page_after: 4,
            },
            BeforeRule {
                page_before: 4,
                page_after: 5,
            },
        ];

        assert_eq!(update.reorder_with_rules(&rules), false);
        assert_eq!(update.pages, seed_pages);
    }

    #[test]
    fn test_update_reorder_some_violations() {
        let mut update = Update {
            pages: vec![1, 2, 3, 4, 5],
        };
        let rules = vec![
            BeforeRule {
                page_before: 2,
                page_after: 1,
            },
            BeforeRule {
                page_before: 2,
                page_after: 3,
            },
            BeforeRule {
                page_before: 5,
                page_after: 4,
            },
            BeforeRule {
                page_before: 5,
                page_after: 1,
            },
            BeforeRule {
                page_before: 20,
                page_after: 50,
            },
            BeforeRule {
                page_before: 45,
                page_after: 6,
            },
        ];

        assert_eq!(update.reorder_with_rules(&rules), true);
        assert_eq!(update.pages, vec![2, 5, 3, 1, 4]);
    }
}
