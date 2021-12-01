/// https://adventofcode.com/2020/day/N
/// ADI: https://adventofcode.com/2020/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2020/leaderboard/private/view/951754 
/// 
/// https://docs.rs/regex/1.4.2/regex/
/// https://docs.rs/regex/1.4.2/regex/#syntax 
extern crate regex;
// use self::regex::{Captures, Regex};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day2)]
pub fn gen1(input: &str) -> Vec<u16> {
    input.lines()
        .map(|line|line.parse().unwrap())
        .collect()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day2, part1)]
pub fn part1(input: &Vec<u16>) -> usize {
    0
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<u16>) -> usize {
    0
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_part1() {
        assert_eq!(888, 999);
    }

    #[test]
    fn test_ex2_part2() {
        assert_eq!(888, 999);
    }

const EX1: &'static str =
r"
";

const EX2: &'static str =
r"
";

}