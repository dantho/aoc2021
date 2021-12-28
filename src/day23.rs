/// https://adventofcode.com/2021/day/23
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use self::Amphipod::*;
use self::GameSpace::*;
use std::collections::HashMap;
use std::fmt;

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day23, part1)]
pub fn gen1(input: &str) -> Vec<Amphipod> {
    input.lines().skip(2).take(2)
    .map(|line|line[3..10].split("#")
        .map(|amphi| match amphi {
            "A" => A,
            "B" => B,
            "C" => C,
            "D" => D,
            wrong => panic!("This is wrong: {}", wrong),
        })).flatten().collect::<Vec<_>>()
}

#[aoc_generator(day23, part2)]
pub fn gen2(input: &str) -> Vec<Amphipod> {
    let orig = gen1(input);
    let insert2rows = vec![D,C,B,A,D,B,A,C];
    orig.iter()
    .take(4)
    .chain(insert2rows.iter())
    .chain(orig.iter().skip(4).take(4))
    .map(|ptr|*ptr)
    .collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day23, part1)]
pub fn part1(input: &[Amphipod]) -> usize {
    assert_eq!(input.len(), 8);
    0
}

#[aoc(day23, part2)]
pub fn part2(input: &[Amphipod]) -> usize {
    assert_eq!(input.len(), 16);
    let mut spaces:HashMap<(usize,usize),GameSpace> = vec![
        (1,2),
        (1,4),
        (1,6),
        (1,8),
        (2,2),
        (2,4),
        (2,6),
        (2,8),
        (3,2),
        (3,4),
        (3,6),
        (3,8),
        (4,2),
        (4,4),
        (4,6),
        (4,8),
    ].into_iter().zip(input.iter())
    .map(|((y,x),amphi)| ((y,x),Amphi(*amphi)))
    .chain(vec![
        ((0,0),Empty),
        ((0,1),Empty),
        ((0,3),Empty),
        ((0,5),Empty),
        ((0,7),Empty),
        ((0,9),Empty),
        ((0,10),Empty),
    ].into_iter())
    .collect();
    println!("{}", GameBoard {spaces}
);

    0
}

struct GameBoard {
    spaces: HashMap<(usize,usize), GameSpace>,
}

impl fmt::Display for GameBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = [[Blank;13];7];
        board[0] = [Wall;13];
        board[1][0] = Wall;
        board[1][3] = Empty;
        board[1][5] = Empty;
        board[1][7] = Empty;
        board[1][9] = Empty;
        board[1][12] = Wall;
        board[2][0] = Wall;
        board[2][1] = Wall;
        board[2][11] = Wall;
        board[2][12] = Wall;
        for row in 2..6 {
            board[row][2] = Wall;
            board[row][4] = Wall;
            board[row][6] = Wall;
            board[row][8] = Wall;
            board[row][10] = Wall;
        }
        board[6] = [Blank,Blank,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Blank,Blank];
        for ((y,x),gs) in self.spaces.iter() {
            board[y+1][x+1] = *gs;
        }
        for y in 0..7 {
            for x in 0..13 {
                write!(f, "{}", board[y][x])?;
            }
            write!(f, "\n")?;
        }        
        write!(f, "\n")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
}
impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match *self {
            A => "A",
            B => "B",
            C => "C",
            D => "D",
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GameSpace {
    Blank,
    Wall,
    Empty,
    Amphi(Amphipod),
}

impl fmt::Display for GameSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Blank => " ".to_string(),
            Wall => "#".to_string(),
            Empty => ".".to_string(),
            Amphi(amphi) => amphi.to_string(),
        })
    }
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gen() {
        let g = gen1(EX1);
        assert_eq!(g.len(), 999);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 999);
    }

const EX1: &'static str =
r"
";

const EX2: &'static str =
r"
";

}