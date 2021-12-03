use std::collections::binary_heap;

/// https://adventofcode.com/2021/day/3
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
#[aoc_generator(day3)]
pub fn gen1(input: &[u8]) -> Vec<Vec<u32>> {
    let input: Vec<u32> = input.iter()
        .map(|num| match num {
            48 => 0u32,
            49 => 1,
            10 => 10,
            _ => panic!("Unrecognized input"),
        }).collect();
    let input: Vec<Vec<u32>> = input.split(|&num| num == 10)
        .map(|s|s.to_vec())
        .collect();
    input
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day3, part1)]
pub fn part1(input: &[Vec<u32>]) -> u32 {
    let len = input.len() as u32;
    let sum_of_1s = input.iter()
        .fold(vec![0;input[0].len()],|sum, v|sum.iter().zip(v.iter()).map(|(s,v)|s+v).collect());
    let gamma = sum_of_1s.iter().fold(0,|word,n|word*2+n*2/len);
    let bitmask = (1<<input[0].len())-1;
    let epsilon = (!gamma)&bitmask;
    println!("Input length: {}", len);
    println!("Sum of 1's: {:?}", sum_of_1s);
    println!("Gamma rate: {}", gamma);
    println!("Epsilon rate: {}", epsilon);
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn part2(input: &[Vec<u32>]) -> u32 {
    let len = input.len() as u32;
    let sum_of_1s = input.iter()
        .fold(vec![0;input[0].len()],|sum, v|sum.iter().zip(v.iter()).map(|(s,v)|s+v).collect());
    let bit_criteria_value = sum_of_1s.iter().fold(0,|crit,s|crit*2+s*2/len);
    let bitmask = (1<<input[0].len())-1;
    let binary_input: Vec<u32> = input.iter()
        .map(|v|v.iter().fold(0,|bin,bit|bin*2+bit)).collect();
    let oxy_gen = binary_input.iter().filter(|&&n|n>bit_criteria_value).min().unwrap();
    let c02scrub = binary_input.iter().filter(|&&n|(!n)&bitmask<bit_criteria_value).max().unwrap();
    println!("BinaryInput: {:?}", binary_input);
    println!("Bit criteria value: {}", bit_criteria_value);
    println!("Oxygen generator rating: {}", oxy_gen);
    println!("C02 scrubber rating: {}", c02scrub);
    let ans = oxy_gen * c02scrub;
    assert!(ans < 14405818, "Answer {} is too high", ans);
    ans
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
        let input = vec![
            vec![0,0,1,0,0],
            vec![1,1,1,1,0],
            vec![1,0,1,1,0],
            vec![1,0,1,1,1],
            vec![1,0,1,0,1],
            vec![0,1,1,1,1],
            vec![0,0,1,1,1],
            vec![1,1,1,0,0],
            vec![1,0,0,0,0],
            vec![1,1,0,0,1],
            vec![0,0,0,1,0],
            vec![0,1,0,1,0],
        ];
        let ans = part2(&input);
        assert_eq!(ans, 999);
    }

const EX1: &'static str =
r"
";

const EX2: &'static str =
r"
";

}