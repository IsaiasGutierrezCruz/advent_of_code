#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>> {
    let mut base_vec = vec![];

    input.lines().for_each(|line| {
        let values = line.split_whitespace();
        base_vec.push(values.map(|x| x.parse::<usize>().unwrap()).collect());
    });
    base_vec
}

// Problem. Part 1.
// The unusual data (your puzzle input) consists of many reports, one report per line. Each report is a list of numbers called levels that are separated by spaces. For example:

// 7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9
// This example data contains six reports each containing five levels.

// The engineers are trying to figure out which reports are safe. The Red-Nosed reactor safety systems can only tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both of the following are true:

// The levels are either all increasing or all decreasing.
// Any two adjacent levels differ by at least one and at most three.
// In the example above, the reports can be found safe or unsafe by checking those rules:

// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
// So, in this example, 2 reports are safe.

// Analyze the unusual data from the engineers. How many reports are safe?

#[aoc(day2, part1)]
pub fn solve_part_01(input: &Vec<Vec<usize>>) -> usize {
    let reports = input.clone();
    let mut safe_reports = 0;

    for report in reports {
        let mut is_safe = true;
        let mut index = 1;
        let is_decreasing: bool = report[0] > report[1];
        while is_safe && index < report.len() {
            let diff = if is_decreasing {
                report[index - 1].checked_sub(report[index])
            } else {
                report[index].checked_sub(report[index - 1])
            };

            match diff {
                Some(d) if d >= 1 && d <= 3 => index += 1,
                _ => is_safe = false,
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }

    safe_reports
}

// --- Part Two ---
// The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the Problem Dampener.

// The Problem Dampener is a reactor-mounted module that lets the reactor safety systems tolerate a single bad level in what would otherwise be a safe report. It's like the bad level never happened!

// Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the report instead counts as safe.

// More of the above example's reports are now safe:

// 7 6 4 2 1: Safe without removing any level.
// 1 2 7 8 9: Unsafe regardless of which level is removed.
// 9 7 6 2 1: Unsafe regardless of which level is removed.
// 1 3 2 4 5: Safe by removing the second level, 3.
// 8 6 4 4 1: Safe by removing the third level, 4.
// 1 3 6 7 9: Safe without removing any level.
// Thanks to the Problem Dampener, 4 reports are actually safe!

// Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports. How many reports are now safe?

fn unsafe_indexes(report: &Vec<usize>) -> Vec<usize> {
    let mut index = 1;
    let is_decreasing: bool = report[0] > report[1];
    let mut bad_level_indexes: Vec<usize> = Vec::new();

    while index < report.len() {
        let diff = if is_decreasing {
            report[index - 1].checked_sub(report[index])
        } else {
            report[index].checked_sub(report[index - 1])
        };

        match diff {
            Some(d) if d >= 1 && d <= 3 => index += 1,
            _ => {
                bad_level_indexes.push(index);
                bad_level_indexes.push(index - 1);
                index += 1;
            }
        }
    }
    bad_level_indexes
}

#[aoc(day2, part2)]
pub fn solve_part_02(input: &Vec<Vec<usize>>) -> usize {
    let reports = input.clone();
    let mut safe_reports = 0;

    for report in reports {
        let bad_indexes = unsafe_indexes(&report);
        if bad_indexes.is_empty() {
            safe_reports += 1;
        } else {
            for index in bad_indexes {
                let mut new_report = report.clone();
                new_report.remove(index);
                if unsafe_indexes(&new_report).is_empty() {
                    safe_reports += 1;
                    break;
                }
            }
        }
    }
    safe_reports
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 2)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(DATA)), 4)
    }
}
