/// https://adventofcode.com/2020/day/17
/// ADI: https://adventofcode.com/2020/leaderboard/private/view/380786 
/// TER: https://adventofcode.com/2020/leaderboard/private/view/951754 
/// 
extern crate crossterm;

use self::crossterm::{QueueableCommand, cursor};

use self::Cube::*;
use std::fmt;
use std::io::{Write, stdout};
const ESC_CLS:&'static str = "\x1B[2J";

// ********************
// *** Generator(s) ***
// ********************
#[aoc_generator(day22)]
pub fn gen1(input: &str) -> ExES {
    ExES::new(input)
}

// *********************
// *** Part1 & Part2 ***
// *********************

// #[aoc(day22, part1)]
// pub fn part1(input: &ExES) -> usize {
//     let mut input = input.clone();
//     println!("{}", input);
//     for i in 0..6 {
//         input = input.eval();
//         println!("{}", input);
//     }

//     // How many cubes end up Active?
//     let ans = input.cubes.iter().map(|plane|plane.iter()).flatten().map(|row|row.iter()).flatten().filter(|cube|**cube==Active).count();

//     // Hacky way to assert on official input only:
//     if input.cubes[0][0].len() == 2 {assert_eq!(ans, 384);}
//     ans
// }

#[aoc(day22, part2)]
pub fn part2(input: &ExES) -> usize {
    let mut input = input.clone();
    println!("------------------");
    println!("----- Part 2 -----");
    println!("------------------");
    for i in 0..6 {
        input = input.eval();
        println!("{}", input);
    }

    // How many cubes end up Active?
    let ans = input.cubes.iter().map(|volume|volume.iter()).flatten().map(|plane|plane.iter()).flatten().map(|row|row.iter()).flatten().filter(|cube|**cube==Active).count();

    // Hacky way to assert on official input only:
    if input.cubes[0][0][0].len() == 2 {assert_eq!(ans, 384);}
    ans
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Cube {
    Inactive,
    Active,
}

// Ex_perimental Energy System
#[derive(Eq, PartialEq, Clone, Hash)]
pub struct ExES {
    cubes: Vec<Vec<Vec<Vec<Cube>>>>,
}

impl From<char> for Cube {
    fn from(c: char) -> Self {
        match c {
            '.' => Inactive,
            '#' => Active,
            _ => panic!("There's some dark matter here! ({})", c),
        }
    }
}

impl fmt::Display for Cube {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 48-56 where
        // 48 is grey, 
        // 49 is grey, 
        // 50 is red, 
        // 51 is green
        // 52 is yellow
        // 53 is blue
        // 54 is violet
        // 55 is cyan, 
        // 56 is bright white, 
        let color= match *self {
            Inactive => 50,
            Active => 51,
        };
        let color=format!("\x1B[{}m", 41 + color);
        write!(
            f,
            "{}{}",
            color,
            match *self {
                Inactive => '.', // '□'
                Active => '☻',
            }
        )?;
        Ok(())
    }
}

impl fmt::Display for ExES {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        // Clear Screen
        print!("{}", ESC_CLS);

        // Position cursor
        stdout()
        .queue(cursor::Hide).unwrap()
        .queue(cursor::MoveTo(0,0)).unwrap()
        // .queue(cursor::Show).unwrap()
        ;

        // Print 3-space
        for plane in &self.cubes[self.cubes.len()/2][0..self.cubes.len()] {
            for row in &plane[0..plane.len()] {
                for cube in &row[0..row.len()] {
                    write!(f, "{}", cube)?;
                }
                writeln!(f, "")?;
            }
            writeln!(f, "")?;
            writeln!(f, "")?;
        }

        // add a delay
        use std::{thread, time};
        let ten_millis = time::Duration::from_millis(200);
        let now = time::Instant::now();
        thread::sleep(ten_millis);
        assert!(now.elapsed() >= ten_millis);        

        Ok(())
    }
}

impl ExES {
    fn new(input: &str) -> Self {
        let cubes = [[
            input
            .lines()
            .filter(|l|l.len() > 0)
            .map(|l| l.chars().map(|c| Cube::from(c)).collect::<Vec<_>>())
            .collect::<Vec<_>>()
        ].to_vec()].to_vec();
        ExES {cubes}.pad()
    }

    fn eval(mut self) -> Self {
        // Expand the playing field BEFORE cloning
        self = self.pad();
        // We will mutate in-place and use this var as orig state for rule eval
        // Might have been clearer to create/return the new state.  Next time.
        let before = self.clone();

        // *** Rules
        // ***    of the
        // ***       Game
        // ***    of
        // *** ExES

        // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active.
        //     Otherwise, the cube becomes inactive.
        // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active.
        //     Otherwise, the cube remains inactive.
        for v in 1..before.cubes.len() - 1 {
            for p in 1..before.cubes[0].len() - 1 {
                for r in 1..before.cubes[0][0].len() - 1 {
                    for c in 1..before.cubes[0][0][0].len() - 1 {
                        let mut count = 0u8;
                        for vv in v - 1..=v + 1 {
                            for pp in p - 1..=p + 1 {
                                for rr in r - 1..=r + 1 {
                                    for cc in c - 1..=c + 1 {
                                        if vv == v && pp == p && rr == r && cc == c {
                                            continue;
                                        };
                                        if before.cubes[vv][pp][rr][cc] == Active {
                                            count += 1;
                                        }
                                    }
                                }
                            }
                        }
                        assert!(count < 81);
                        let changing_cube = &mut self.cubes[v][p][r][c];
                        *changing_cube = match *changing_cube {
                            Active => {
                                if count == 2 || count == 3 {
                                    Active
                                } else {
                                    Inactive
                                }
                            },
                            Inactive => {
                                if count == 3 {
                                    Active
                                } else {
                                    Inactive
                                }
                            },
                        }
                    }
                }
            }
        }
        self
    }

    fn pad(mut self) -> Self {
        for volume in &mut self.cubes {
            for area in volume {
                for row in area {
                    row.reverse();
                    row.push(Inactive);
                    row.reverse();
                    row.push(Inactive);
                }
            }
        }
        let pad_row: Vec<_> = self.cubes[0][0][0].iter().map(|_| Inactive).collect();
        for volume in &mut self.cubes {
            for area in volume {
                area.reverse();
                area.push(pad_row.clone());
                area.reverse();
                area.push(pad_row.clone());
            }
        }
        let pad_area: Vec<_> = {
            self.cubes[0][0].iter().map(|_| pad_row.clone()).collect()
        };
        for volume in &mut self.cubes {
            volume.reverse();
            volume.push(pad_area.clone());
            volume.reverse();
            volume.push(pad_area.clone());
        }
        let pad_volume: Vec<_> = {
            self.cubes[0].iter().map(|_| pad_area.clone()).collect()
        };
        self.cubes.reverse();
        self.cubes.push(pad_volume.clone());
        self.cubes.reverse();
        self.cubes.push(pad_volume);
        self
    }
}

// // See https://jrgraphix.net/r/Unicode/2700-27BF for Dingbats in unicode
// impl TileID {
//     fn to_char(&self) -> char {
//         match *self {
//             Inactive => ' ',
//             Wall => '■',
//             Block => '□',
//             HorizontalPaddle => '═',
//             Ball => '☻', // '●',
//         }
//     }
// }
fn print(s: &str) {
    print!("{}",s);
    stdout().flush().unwrap();
}
fn print_ch(ch: char) {
    print!("{}",ch);
    stdout().flush().unwrap();
}
fn set_cursor_pos(y:isize,x:isize) {
    print!("\x1B[{};{}H", y+1, x+1);
    stdout().flush().unwrap();
}
fn set_color(color:u8) {
    print(
        &format!("\x1B[{}m", 41 + color)
    );
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_ex1_part1() {
    //     assert_eq!(part1(&gen1(EX1)), 112);
    // }

    #[test]
    fn test_ex1_part2() {
        assert_eq!(part2(&gen1(EX1)), 848);
    }

    const EX1: &'static str = r"
.#.
..#
###
";
}
