use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let inp = Input::from_str(input);
    let num_safe_reports = inp.reports.iter().filter(|r| r.is_safe()).count();
    Some(num_safe_reports as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let inp = Input::from_str(input);
    let num_safe_reports = inp
        .reports
        .par_iter()
        .filter(|r| r.is_safe_with_problem_dampener())
        .count();
    Some(num_safe_reports as u32)
}

struct Input {
    pub(crate) reports: Vec<Report>,
}

impl Input {
    fn from_str(input: &str) -> Input {
        let reports: Vec<Report> = input.lines().map(|line| Report::from_str(line)).collect();
        Input { reports }
    }
}

struct Report {
    levels: Vec<u32>,
}

impl Report {
    pub fn from_str(input: &str) -> Self {
        Report {
            levels: input
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect(),
        }
    }

    pub fn is_safe(&self) -> bool {
        Self::check_safe(&self.levels)
    }

    fn check_safe(levels: &[u32]) -> bool {
        if levels.len() < 2 {
            return true;
        }

        let mut increasing = None;

        for (&prev, &current) in levels.iter().zip(levels.iter().skip(1)) {
            let diff = prev.abs_diff(current);
            if diff < 1 || diff > 3 {
                return false;
            }

            increasing.get_or_insert(current > prev);

            if let Some(is_increasing) = increasing {
                if (is_increasing && current < prev) || (!is_increasing && current > prev) {
                    return false;
                }
            }
        }
        true
    }

    pub fn is_safe_with_problem_dampener(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut new_lvls = self.levels.clone();
            new_lvls.remove(i);
            if Self::check_safe(&new_lvls) {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAFE_REPORT_INCREASING: &str = "1 2 4 6 9";
    const SAFE_REPORT_DECREASING: &str = "8 6 4 3 2";

    const UNSAFE_REPORT_INCREASING_AND_DECREASING: &str = "2 3 4 3 2";
    const UNSAFE_REPORT_REPEATED_NUMBER: &str = "5 3 2 3 4 6 6 5 4";
    const UNSAFE_REPORT_MORE_THAN_THREE_INCREASING: &str = "1 2 9";
    const UNSAFE_REPORT_MORE_THAN_THREE_DECREASING: &str = "8 4 3 2 1";

    #[test]
    fn test_safe_report_increasing() {
        let report = Report::from_str(SAFE_REPORT_INCREASING);
        assert!(report.is_safe());
    }

    #[test]
    fn test_safe_report_decreasing() {
        let report = Report::from_str(SAFE_REPORT_DECREASING);
        assert!(report.is_safe());
    }

    #[test]
    fn test_unsafe_report_increasing_and_decreasing() {
        let report = Report::from_str(UNSAFE_REPORT_INCREASING_AND_DECREASING);
        assert!(!report.is_safe());
    }

    #[test]
    fn test_unsafe_report_repeated_number() {
        let report = Report::from_str(UNSAFE_REPORT_REPEATED_NUMBER);
        assert!(!report.is_safe());
    }

    #[test]
    fn test_unsafe_report_more_than_three_increasing() {
        let report = Report::from_str(UNSAFE_REPORT_MORE_THAN_THREE_INCREASING);
        assert!(!report.is_safe());
    }

    #[test]
    fn test_unsafe_report_more_than_three_decreasing() {
        let report = Report::from_str(UNSAFE_REPORT_MORE_THAN_THREE_DECREASING);
        assert!(!report.is_safe());
    }

    #[test]
    fn test_part_one_simple() {
        let input = r#"

            1 2 3 4
            6 4 2 1
            2 2 3 3
            1 2 3 2 1
            1 2 5 9

        "#
        .trim();

        let result = part_one(input);
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_with_dampener_is_all_safe() {
        let input = r#"

            1 2 3 4
            6 4 2 1
            2 2 3 4
            1 2 6 4
            1 2 3 2

        "#
        .trim();

        let result = part_two(input);
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two_even_with_dampener_is_not_all_safe() {
        let input = r#"

            1 5 9
            6 4 2 1
            1 2 2 2

        "#
        .trim();

        let result = part_two(input);
        assert_eq!(result, Some(1));
    }
}
