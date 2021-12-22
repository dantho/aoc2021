/// https://adventofcode.com/2021/day/21
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day21)]
pub fn gen1(_input: &str) -> () {
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day21, part1)]
pub fn part1(_input: &()) -> usize {
    play (100,10,7)
    // 906093 is the answer!
}

#[aoc(day21, part2)]
pub fn part2(_input: &()) -> usize {
    play (3,10,7)
}

pub fn play(max_roll: usize, starting_pos1: usize, starting_pos2: usize) -> usize {
    let game_board = 10;
    let mut player_pos = [starting_pos1, starting_pos2];
    let mut scores = [0; 2];
    let mut die = 0usize; // Not valid roll - initialization only
    let mut roll_count = 0usize;
    'game_loop:
    loop {
        for player in 0..=1 {
            let roll_x3: usize = ((0..3).into_iter().map(|_|{
                roll_count += 1;
                die = die % max_roll + 1;
                die
            }).sum::<usize>() - 1) % max_roll + 1;
            let pos = &mut player_pos[player];
            *pos = ((*pos + roll_x3) - 1) % game_board + 1;
            scores[player] += *pos;
            println!{"Player {} rolls {} and moves to space {} for a total score of {}."
            , player+1, roll_x3, *pos, scores[player]}; 
            if scores[player] >= 1000 {break 'game_loop};
        }
    }
    let winner = if scores[0] >= 1000 {0} else {1};
    let loser = (winner + 1) % 2;
    println!{"WINNER --> Player {} with a score of {}.",winner+1,scores[winner]}; 
    println!{"Loser ---> Player {} had a score of {}.",loser+1,scores[loser]}; 
    println!("The die was rolled {} times", roll_count);

    roll_count * scores[loser]
}

// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ex1_play() {
        let res = play(100,4,8);
        assert_eq!(res, 739785);
    }

// const EX1: &'static str =
// r"
// ";

// const EX2: &'static str =
// r"
// ";

}