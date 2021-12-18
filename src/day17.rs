/// https://adventofcode.com/2021/day/17
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day17)]
pub fn gen1(input: &str) -> ((isize,isize),(isize,isize)) {
    // Not worth writing a parser!
    // target area: x=253..280, y=-73..-46
    ((253,280),(-73,-46))
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day17, part1)]
pub fn part1(input: &((isize,isize),(isize,isize))) -> isize {
    let ((tx1,tx2),(ty1,ty2)) = *input;
    let start = (0,0);
    // Find the initial velocity that causes the probe to reach the highest y position
    // and still eventually be within the target area after any step.
    // After far too much linear algebra and calculus, then giving up.  :(
    // By observation:
    //     Step Count to final X position  = (Vx0^2+Vx0) / 2
    // We need Final X to be max within tx1 to tx2 inclusive
    let mut step_min = isize::MAX;
    let mut step_max = 0;
    for vx0 in 1.. {
        let xpos = (vx0*vx0+vx0)/2;
        if xpos > tx1 {
            step_min = vx0.min(step_min);  // First assignment will stick
            step_max = vx0; // last assignment will stick
        } 
        if xpos > tx2 {break;}
    }
    assert!(step_max > 0);
    // Now consider that the y-velocity is vy0 at start and -vy0 on the way down,
    // and it passes through (and exactly hits!) y = 0 (Physics! Math!)
    // so the highest speed that will hit the target (which means highest apex)
    // Was the last single step if target is above zero, or next single step if below
    // Our target is below, vy = -v0+1 = max distance = ty2
    let vy0 = -ty1 -1;
    let ymax = (1..=vy0).sum();
    ymax
}

#[aoc(day17, part2)]
pub fn part2(input: &((isize,isize),(isize,isize))) -> usize {
    // target corners
    let ((tx1,tx2),(ty1,ty2)) = *input;
    let start = (0,0);
    // See part 1 notes here
    let mut step_min = isize::MAX;
    let mut step_max = 0;
    for vx0 in 1.. {
        let xpos = (vx0*vx0+vx0)/2;
        if xpos > tx1 {
            step_min = vx0.min(step_min);  // First assignment will stick
            step_max = vx0; // last assignment will stick
        } 
        if xpos > tx2 {break;}
    }
    assert!(step_max > 0);
    // See part 1 notes for description of vy0_max calculation -- a single step
    // vy0_min calculates maximum integer steps given known min step count from x, above
    let mut total_good_shots = 0;
    for steps in step_min..=step_max {
        let vx0 = steps;
        let xpos = calc_x_pos(steps,steps);
        total_good_shots += (ty1..0).map(|vy0|{
            let ypos=calc_y_pos(steps,vy0);
            (xpos,ypos)
        })
        .filter(|(x,y)|*y>=ty1 && *y<=ty2)
        .count();
    }
    total_good_shots
}
fn in_target(target: &((isize,isize),(isize,isize)), x: isize, y: isize) -> bool {
    let ((tx1,tx2),(ty1,ty2)) = target;
    x>=tx1 && x<=tx2 && y>=ty1 && y<=ty2
}
fn calc_x_pos(steps: isize, vx0: isize) -> isize {
    (0..steps).fold(0,|sum,step|sum+vx0-step)
}
fn calc_y_pos(steps: isize, vy0: isize) -> isize {
    (vy0..vy0+steps).fold(0,|sum,vy|sum+vy)
}
// *************
// *** Tests ***
// *************
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let g = gen1("");
        let p1 = part1(&g);
        assert_eq!(p1, 2628); // my known good answer
    }

    #[test]
    fn test_ex1_part1() {
        let p1 = part1(&EX1);
        assert_eq!(p1, 45);
    }

    #[test]
    fn test_ex1_part2() {
        let p1 = part2(&EX1);
        assert_eq!(p1, 112); 
    }

const EX1: ((isize, isize), (isize, isize)) = ((20,30),(-10,-5));

const EX2: &'static str =
r"
";

}