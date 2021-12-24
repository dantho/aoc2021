/// https://adventofcode.com/2021/day/22
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use std::ops::Sub;

#[derive(Copy,Clone,Debug,PartialEq,Eq)]
pub struct Cuboid ((isize, isize, isize), (isize, isize, isize));
impl Cuboid {
    fn is_cuboid(&self) -> bool {
        let Cuboid ((x0,y0,z0),(x1,y1,z1)) = self;
        x1>=x0 && y1>=y0 && z1>=z0
    }
    fn bounded_by(self, other:Self) -> Option<Self> {
        let Cuboid ((x0,y0,z0),(x1,y1,z1)) = self;
        let Cuboid ((xx0,yy0,zz0),(xx1,yy1,zz1)) = other;
        let new = Cuboid ((x0.max(xx0),y0.max(yy0),z0.max(zz0)),
            (x1.min(xx1),y1.min(yy1),z1.min(zz1)));
        if new.is_cuboid() {Some(new)} else {None}
    }
    fn size(self) -> usize {
        let Cuboid ((x0,y0,z0),(x1,y1,z1)) = self;
        ((x1-x0+1)*(y1-y0+1)*(z1-z0+1)) as usize
    }
}
impl Sub for Cuboid {
    type Output = Vec<Self>;
    fn sub(self, other:Self) -> Self::Output {
        let Cuboid ((x0,y0,z0),(x1,y1,z1)) = self;
        let Cuboid ((xx0,yy0,zz0),(xx1,yy1,zz1)) = other;
        vec![
            Cuboid ((x0,y0,z0),(x1,y1,zz0-1)),
            Cuboid ((x0,y0,zz1+1),(x1,y1,z1)),
            Cuboid ((x0,y0,zz0),(x1,yy0-1,zz1)),
            Cuboid ((x0,yy1+1,zz0),(x1,y1,zz1)),
            Cuboid ((x0,yy0,zz0),(xx0-1,yy1,zz1)),
            Cuboid ((xx1+1,yy0,zz0),(x1,yy1,zz1)),
        ].into_iter()
        .filter(|c|c.is_cuboid())
        .filter_map(|c|c.bounded_by(self))
        .collect::<Vec<_>>()
    }
}

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day22)]
pub fn gen1(input: &str) -> Vec<(bool,Cuboid)> {
    input.lines()
    .filter(|l|l.len()!=0)
    .map(|line| {
        let mut halves = line.split(" ");
        let on_or_off = halves.next().unwrap();
        let is_on = on_or_off == "on";
        if !is_on {assert_eq!(on_or_off,"off");}
        let xyz = halves.next().unwrap().split(",");
        let xyz = xyz.map(|rng_str| {
            let rng_str = rng_str.chars().skip(2).collect::<String>(); // discard
            let rng_v = rng_str.split("..")
            .map(|start_or_end| {
                start_or_end.parse::<isize>().unwrap()
            }).collect::<Vec<_>>();
            (rng_v[0],rng_v[1])
        }).collect::<Vec<_>>();
        (is_on, Cuboid ((xyz[0].0,xyz[1].0,xyz[2].0),(xyz[0].1,xyz[1].1,xyz[2].1)))
    }).collect::<Vec<_>>()
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day22, part1)]
pub fn part1(input: &[(bool,Cuboid)]) -> usize {
    // Now merge in Cuboids one-by-one.  :)
    // Worst case is that a new smaller cuboid is inserted fully inside another, such that the
    // original cuboid's remaining volume must be sliced into are 6 cuboids, one on each side of the
    // inserted cuboid.  We'll slice those 6 sections as follows:
    // o  Two z-sections using full xy of original cuboid being diced.
    // o  Then two y-sections using full x, but a section of z that overlaps the new cube.
    // o  Then two x-sections with both y and z overlapping with the new cube.
    // All other cases are a simple subsets of this worst case.
    // Note that the slicing and dicing happens regardless of the on/off state of either cuboid.
    // Note that the old cuboid(s) are diced, new cuboids "win" -- remain whole until diced by a later cuboid.
    let rebooted = input.iter().map(|ptr|*ptr)
    .filter(|(_,c)|c.0.0 <= 50 && c.0.0 >= -50)
    .fold(Vec::new(),|steps: Vec<(bool,Cuboid)>, (new_on_off, new_c):(bool,Cuboid)| {
        let mut updated = steps.into_iter()
        .map(|(on_off,c):(bool,Cuboid)|{
            (c-new_c).into_iter().map(move |c|(on_off,c))
        })
        .flatten()
        .collect::<Vec<_>>();
        updated.push((new_on_off,new_c));
        updated
    });
    rebooted.into_iter()
    .filter_map(|(on_off,c)|if on_off {Some(c.size())} else {None})
    .sum()
    // 1211172281877240 is too high
    // 11841109137665 is too high
}

#[aoc(day22, part2)]
pub fn part2(_input: &[(bool,Cuboid)]) -> usize {
    0
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
        assert_eq!(g.len(), 4);
        let g = gen1(EX2);
        assert_eq!(g.len(), 22);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 39);
    }

    #[test]
    fn test_ex2_part1() {
        let g = gen1(EX2);
        let p1 = part1(&g);
        assert_eq!(p1, 590784);
    }

    #[test]
    fn test_cuboid_sub() {
        let bigger = Cuboid ((0,0,0),(3,3,3));
        let smaller = Cuboid ((1,1,1),(2,2,2));
        let diff = bigger-smaller;
        assert_eq!(diff.len(), 6);
        let bigger = Cuboid ((0,0,0),(1,1,1));
        let smaller = Cuboid ((1,1,1),(2,2,2));
        let diff = bigger-smaller;
        assert_eq!(diff.len(), 3);
        let bigger = Cuboid ((0,0,0),(1,1,1));
        let smaller = Cuboid ((1,2,2),(2,2,2));
        let diff = bigger-smaller;
        assert_eq!(diff, [bigger]);
        let bigger = Cuboid ((0,0,0),(0,0,0));
        let smaller = Cuboid ((2,2,2),(2,2,2));
        let diff = bigger-smaller;
        assert_eq!(diff, [bigger]);
    }

    #[test]
    fn test_is_cuboid() {
        assert_eq!(true, Cuboid ((0,0,0),(0,0,0)).is_cuboid());
        assert_eq!(true, Cuboid ((0,0,0),(1,1,1)).is_cuboid());
        assert_eq!(true, Cuboid ((1,2,3),(2,3,4)).is_cuboid());
        assert_eq!(false, Cuboid ((0,0,0),(-1,0,0)).is_cuboid());
        assert_eq!(false, Cuboid ((0,1,0),(0,0,0)).is_cuboid());
        assert_eq!(false, Cuboid ((-2,0,0),(-3,0,0)).is_cuboid());
        assert_eq!(false, Cuboid ((1,1,1),(0,0,0)).is_cuboid());
        assert_eq!(true, Cuboid ((isize::MIN,isize::MIN,isize::MIN),(isize::MAX,isize::MAX,isize::MAX)).is_cuboid());
    }

    #[test]
    fn test_cuboid_size() {
        assert_eq!(1, Cuboid ((0,0,0),(0,0,0)).size());
        assert_eq!(2, Cuboid ((0,0,0),(0,0,1)).size());
        assert_eq!(4, Cuboid ((0,0,0),(0,1,1)).size());
        assert_eq!(8, Cuboid ((0,0,0),(1,1,1)).size());
        assert_eq!(100, Cuboid ((0,0,0),(0,9,9)).size());
        assert_eq!(1_000_000, Cuboid ((0,0,0),(99,99,99)).size());
        assert_eq!(1_000_000, Cuboid ((-10,-10,-10),(89,89,89)).size());
        assert_eq!(1_000_000, Cuboid ((-10,-20,-30),(89,79,69)).size());
        assert_eq!(1_030_301, Cuboid ((-50,-50,-50),(50,50,50)).size());
    }

const EX1: &'static str =
r"on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

const EX2: &'static str =
r"on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

}