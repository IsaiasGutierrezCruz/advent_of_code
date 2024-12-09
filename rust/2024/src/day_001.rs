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

    zip(left, right).map(|(l, r)| l.abs_diff(r) ).sum()
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

}