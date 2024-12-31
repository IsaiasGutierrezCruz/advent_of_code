#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

pub const DIRECTIONS: [Point; 8] = [
    Point { x: 1, y: 0 },   // right
    Point { x: 1, y: -1 },  // down-right
    Point { x: 0, y: -1 },  // down
    Point { x: -1, y: -1 }, // down-left
    Point { x: -1, y: 0 },  // left
    Point { x: -1, y: 1 },  // up-left
    Point { x: 0, y: 1 },   // up
    Point { x: 1, y: 1 },   // up-right
];

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

// --- Day 4: Ceres Search ---
// "Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

// As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt; she'd like to know if you could help her with her word search (your puzzle input). She only has to find one word: XMAS.

// This word search allows words to be horizontal, vertical, diagonal, written backwards, or even overlapping other words. It's a little unusual, though, as you don't merely need to find one instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where irrelevant characters have been replaced with .:

// ..X...
// .SAMX.
// .A..A.
// XMAS.S
// .X....
// The actual word search will be full of letters instead. For example:

// MMMSXXMASM
// MSAMXMSMSA
// AMXSXMAAMM
// MSAMASMSMX
// XMASAMXAMM
// XXAMMXXAMA
// SMSMSASXSS
// SAXAMASAAA
// MAMMMXMMMM
// MXMXAXMASX
// In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where letters not involved in any XMAS have been replaced with .:

// ....XXMAS.
// .SAMXMS...
// ...S..A...
// ..A.A.MS.X
// XMASAMX.MM
// X.....XA.A
// S.S.S.S.SS
// .A.A.A.A.A
// ..M.M.M.MM
// .X.X.XMASX
// Take a look at the little Elf's word search. How many times does XMAS appear?

pub fn validate_pattern_in_direction(
    x_limit: i32,
    y_limit: i32,
    point: Point,
    direction: &Point,
    pattern: [char; 4],
) -> bool {
    let steps = pattern.len() as i32 - 1;

    let new_point = Point {
        x: point.x + direction.x * steps,
        y: point.y + direction.y * steps,
    };

    if new_point.x < 0 || new_point.x >= x_limit || new_point.y < 0 || new_point.y >= y_limit {
        return false;
    }
    true
}

#[aoc(day4, part1)]
pub fn solve_part_01(input: &Vec<Vec<char>>) -> usize {
    const PATTERN_TO_SEARCH: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut total_matches = 0;

    let x_size = input.len() as i32;
    let y_size = input[0].len() as i32;

    for row_index in 0..input.len() {
        for col_index in 0..input[row_index].len() {
            let point = Point {
                x: row_index as i32,
                y: col_index as i32,
            };
            for direction in DIRECTIONS.iter() {
                // review if it is possible that the pattern exists in this direction
                if !validate_pattern_in_direction(
                    x_size,
                    y_size,
                    point,
                    direction,
                    PATTERN_TO_SEARCH,
                ) || input[point.x as usize][point.y as usize] != PATTERN_TO_SEARCH[0]
                {
                    // the pattern is not possible in this direction
                    continue;
                }
                // check if the pattern exists in this direction
                let is_second_char = input[(point.x + direction.x * 1) as usize]
                    [(point.y + direction.y * 1) as usize];
                let is_third_char = input[(point.x + direction.x * 2) as usize]
                    [(point.y + direction.y * 2) as usize];
                let is_fourth_char = input[(point.x + direction.x * 3) as usize]
                    [(point.y + direction.y * 3) as usize];

                if is_second_char == PATTERN_TO_SEARCH[1]
                    && is_third_char == PATTERN_TO_SEARCH[2]
                    && is_fourth_char == PATTERN_TO_SEARCH[3]
                {
                    total_matches += 1;
                }
            }
        }
    }
    total_matches
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn sample_01() {
        assert_eq!(solve_part_01(&input_generator(DATA)), 18)
    }

    // #[test]
    // fn sample_02() {
    //     assert_eq!(solve_part_02(&input_generator(DATA_PART_2)), 48)
    // }
}
