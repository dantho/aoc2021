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
#[aoc_generator(day1)]
pub fn gen1(input: &str) -> String {
    input.to_string()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day1, part1)]
pub fn part1(input: &String) -> usize {
    input
}

#[aoc(day1, part2)]
pub fn part2(input: &String) -> usize {
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