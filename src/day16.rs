/// https://adventofcode.com/2021/day/16
/// TER: https://adventofcode.com/2021/leaderboard/private/view/951754 
use self::{Operator::*};

// ********************
// *** Generator(s) ***
// ********************/
#[aoc_generator(day16)]
pub fn gen1(input: &str) -> Packet {
    Packet::parse_hex(input)
}

// *********************
// *** Part1 & Part2 ***
// *********************
#[aoc(day16, part1)]
pub fn part1(input: &Packet) -> usize {
    input.flatten().iter().map(|p|p.version as usize).sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &Packet) -> usize {
    0
}

#[derive(Eq,PartialEq,Copy,Clone,Debug)]
pub enum Operator {
    Literal = 4,
    Op1 = 6,
    OpUnknown = isize::MAX,
}
impl Operator {
    pub fn parse_u8(v: u8) -> Self {
        match v {
            4 => Literal,
            6 => Op1,
            _ => OpUnknown,
        }
    }
}
pub struct Packet {
    version: u8,
    typeid: Operator,
    value: Option<u64>,
    packets: Vec<Packet>,
}
impl<'a> Packet {
    pub fn flatten(&'a self) -> Vec<&'a Self> {
        let mut vecp = vec![self];
        for p in &self.packets {
            vecp.extend_from_slice(&p.flatten());
        }
        vecp
    }
}
impl Packet {
    pub fn is_literal(&self) -> bool {
        self.typeid == Literal && self.value.is_some()
    }
    pub fn is_leaf_node(&self) -> bool {
        self.packets.len() == 0
    }
    fn hex2u8(hex_digit: char) -> u8 {
        match hex_digit {
            d if d>='0' && d<='9' => d as u8 - '0' as u8,
            h if h>='A' && h<='F' => h as u8 - 'A' as u8 + 10,
            _ => panic!("Not a Hex digit"),
        }
    }
    fn u8_to_u64(nibbles: &[u8]) -> u64 {
        let padlen = 16-nibbles.len();
        let mut val_u64 = nibbles.iter()
        .fold(0,|val,&nib|val*16+nib as u64);
        // pad with zeros to 16 nibbles
        for _ in 0..padlen {
            val_u64 *= 16;
        }
        val_u64
    }
    pub fn parse_hex(hex_str:&str) -> Self {
        let v_char: Vec<u8> = hex_str.chars()
        .map(|h|Self::hex2u8(h)).collect();
        let mut v_u64: Vec<u64> = v_char.chunks(16)
        .map(|chunk|Self::u8_to_u64(&chunk.iter().map(|ptr|*ptr).collect::<Vec<u8>>()))
        .collect();
        if cfg!(test) {println!("{:?}",v_u64)};
        Self::parse_u64s(&mut v_u64)
    }
    fn parse_u64s(mut v: &mut Vec<u64>) -> Packet {
        let version = Self::left_shift(&mut v, 3) as u8;
        let typeidnum = Self::left_shift(&mut v, 3) as u8;
        let typeid = Operator::parse_u8(typeidnum);
        match typeid {
            Literal => {
                let mut val = 0;
                loop {
                    val *= 16;
                    let isLast = Self::left_shift(&mut v, 1) == 0;
                    let nibble = Self::left_shift(&mut v, 4);
                    val += nibble;
                    if isLast {break;}
                };
                Packet {version, typeid, value: Some(val), packets: Vec::new()}
            },
            _ => {
                let mut packets: Vec<Packet> = Vec::new();
                let length_type_id = Self::left_shift(&mut v, 1);
                match length_type_id {
                    0 => {
                        let total_bit_length = Self::left_shift(&mut v, 15);
                        let packet_bits = Self::left_shift(&mut v, total_bit_length);
                        let packet_bits = packet_bits<<(64-total_bit_length); // MSB justified
                        let mut vv = vec![packet_bits];
                        loop {
                            packets.push(Self::parse_u64s(&mut vv));
                            if vv.is_empty() {break;}
                            if vv[0] == 0 {break;}
                        }
                    },
                    1 => {
                        let sub_packet_cnt = Self::left_shift(&mut v, 11);
                        for _sp in 0..sub_packet_cnt {
                            packets.push(Self::parse_u64s(&mut v));
                            if v.is_empty() {break;}
                            if v[0] == 0 {break;}
                        }
                    },
                    _ => panic!("This can't happen with 1 bit")
                }
                // clean up remainder of input vector
                loop {
                    if v.is_empty() {break;}
                    if v[v.len()-1] == 0 {
                        v.pop();
                    } else {break;}
                }
                Packet {version, typeid, value: None, packets}
            }

        }
    }
    fn left_shift(v: &mut Vec<u64>, bit_cnt: u64) -> u64 {
        assert!(bit_cnt > 0);
        assert!(bit_cnt < 64);
        v.reverse(); // LSWord first
        let shifted_out_bits = v.iter_mut()
        .fold(0,|carry_bits, word|{
            let shifted = *word >> (64-bit_cnt);
            *word = (*word << bit_cnt) + carry_bits;
            shifted
        });
        v.reverse();
        shifted_out_bits
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
        assert_eq!(g.version, 6);
        assert_eq!(g.typeid, Literal);
        assert_eq!(g.typeid as u8, 4);
    }

    #[test]
    fn test_ex1_part1() {
        let g = gen1(EX1);
        let p1 = part1(&g);
        assert_eq!(p1, 6);
    }
    #[test]
    fn test_ex2_part1() {
        let g = gen1(EX2);
        let p1 = part1(&g);
        assert_eq!(p1, 9);
    }
    #[test]
    fn test_ex3_part1() {
        let g = gen1(EX3);
        let p1 = part1(&g);
        assert_eq!(p1, 14);
    }
    #[test]
    fn test_ex4_part1() {
        let g = gen1(EX4);
        let p1 = part1(&g);
        assert_eq!(p1, 16);
    }
    #[test]
    fn test_ex5_part1() {
        let g = gen1(EX5);
        let p1 = part1(&g);
        assert_eq!(p1, 12);
    }
    #[test]
    fn test_ex6_part1() {
        let g = gen1(EX6);
        let p1 = part1(&g);
        assert_eq!(p1, 23);
    }
    // #[test]
    // fn test_ex7_part1() {
    //     let g = gen1(EX7);
    //     let p1 = part1(&g);
    //     assert_eq!(p1, 31);
    // }

// 110100101111111000101000
// VVVTTTAAAAABBBBBCCCCC
const EX1: &'static str =
r"D2FE28";
const EX2: &'static str =
r"38006F45291200";
const EX3: &'static str =
r"EE00D40C823060";
const EX4: &'static str =
r"8A004A801A8002F478";
const EX5: &'static str =
r"620080001611562C8802118E34";
const EX6: &'static str =
r"C0015000016115A2E0802F182340";
const EX7: &'static str =
r"A0016C880162017C3686B18A3D4780";

}