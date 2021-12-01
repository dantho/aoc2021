pub mod day12;
use day12::*;

fn main() {
    assert_eq!(&part1(&gen1(&EX1)), 999);
}

const EX1: &'static str = r"
F10
N3
F7
R90
F11";
