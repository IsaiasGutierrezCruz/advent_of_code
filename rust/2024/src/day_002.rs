#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<usize>>  {
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

    // #[test]
    // fn sample_02() {
    //     assert_eq!(solve_part_02(&input_generator(DATA)), 31)
    // }
}
