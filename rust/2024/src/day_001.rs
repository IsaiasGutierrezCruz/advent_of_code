use std::collections::HashMap;
use std::iter::zip;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left = vec![];
    let mut right = vec![];

    input.lines().for_each(|line| {
        let mut values = line.split_whitespace();

        left.push(values.next().unwrap().parse::<usize>().unwrap());
        right.push(values.next().unwrap().parse::<usize>().unwrap());
    });

    (left, right)
}

#[aoc(day1, part1)]
pub fn solve_part_01(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let (mut left, mut right) = input.clone();

    left.sort();
    right.sort();

    zip(left, right).map(|(l, r)| l.abs_diff(r)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part_02(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let (left, right) = input.clone();
    let mut scores_cache: HashMap<usize, usize> = HashMap::new();
    let mut total_scores: Vec<usize> = Vec::new();

    for right_item in right {
        scores_cache
            .entry(right_item)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for left_item in left {
        match scores_cache.get(&left_item) {
            Some(score) => {
                total_scores.push(left_item * score);
            }
            None => {
                total_scores.push(0);
            }
        }
    }

    total_scores.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "1   3
2   3
3   3
3   4
3   5
4   9";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 11)
    }

    #[test]
    fn sample_02() {
        assert_eq!(solve_part_02(&input_generator(DATA)), 31)
    }
}
