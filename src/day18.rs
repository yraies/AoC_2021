use std::fmt::Display;
use std::ops::Add;
use std::str;

use itertools::Itertools;
use pom::parser::Parser;
use pom::parser::*;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day18.txt").expect("Could not find day 18 data!");
    let _testinput = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SNumber {
    Pair(Box<(SNumber, SNumber)>),
    Number(u8),
}
impl Display for SNumber {
    fn fmt(&self, w: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            SNumber::Pair(p) => {
                write!(w, "[")?;
                p.0.fmt(w)?;
                write!(w, ",")?;
                p.1.fmt(w)?;
                write!(w, "]")
            }
            SNumber::Number(nr) => write!(w, "{}", *nr),
        }
    }
}

//#[aoc_generator(day)]
pub fn parse_data<'a>(input: &'a str) -> Vec<SNumber> {
    fn parse_snumber<'a>() -> Parser<'a, u8, SNumber> {
        let number = one_of(b"0123456789").map(|nr| SNumber::Number(nr - b'0'));
        let tup = (sym(b'[') * call(parse_snumber) - sym(b',') + call(parse_snumber) - sym(b']'))
            .map(|f| SNumber::Pair(Box::new((f.0, f.1))));
        number | tup
    }

    input
        .lines()
        .map(|line| parse_snumber().parse(line.as_bytes()).unwrap())
        .collect_vec()
}

impl Add for SNumber {
    type Output = SNumber;

    fn add(self, rhs: Self) -> Self::Output {
        //println!("Adding {} + {}", self, rhs);
        SNumber::Pair(Box::new((self, rhs))).reduced()
    }
}

impl SNumber {
    fn reduced(mut self) -> SNumber {
        loop {
            //println!("Reducing!");
            if self.try_explode() {
                continue;
            }
            if self.try_split() {
                continue;
            }
            break;
        }

        self
    }

    fn try_split(&mut self) -> bool {
        match self {
            SNumber::Pair(p) => p.0.try_split() || p.1.try_split(),
            SNumber::Number(nr) => {
                let n = *nr;
                if n >= 10 {
                    *self = SNumber::Pair(Box::new((
                        SNumber::Number(n / 2),
                        SNumber::Number(n / 2 + n % 2),
                    )));
                    //println!("Split into {}", self);
                    return true;
                }
                return false;
            }
        }
    }

    fn try_explode(&mut self) -> bool {
        let (left, deep_pair, right) =
            self.try_get_first_deep_pair_with_neighbours(0, None, None, None);
        match deep_pair {
            Some(snum) => {
                //println!("Trying to explode with {:?}, {:?}, {:?}", left, snum, right);
                if let SNumber::Pair(p) = snum {
                    let (exp_l, exp_r) = p.as_mut();
                    if let (Some(l), &mut Self::Number(exp)) = (left, exp_l) {
                        *l = exp + *l;
                    }
                    if let (Some(r), &mut Self::Number(exp)) = (right, exp_r) {
                        *r = exp + *r;
                    }
                }

                *snum = SNumber::Number(0);

                true
            }
            _ => false,
        }
    }

    fn try_get_first_deep_pair_with_neighbours<'a>(
        &'a mut self,
        depth: usize,
        last_left: Option<&'a mut u8>,
        last_pair: Option<&'a mut SNumber>,
        last_right: Option<&'a mut u8>,
    ) -> (Option<&mut u8>, Option<&mut SNumber>, Option<&mut u8>) {
        if last_pair.is_none() && depth >= 4 {
            if let SNumber::Pair(_) = self {
                return (last_left, Some(self), None);
            }
        }

        match self {
            SNumber::Pair(p) => match (last_left, last_pair, last_right) {
                (l, Some(cv), Some(cr)) => (l, Some(cv), Some(cr)),
                (l, c, r) => {
                    let (upd_left, upd_pair, upd_right) =
                        p.0.try_get_first_deep_pair_with_neighbours(depth + 1, l, c, r);
                    p.1.try_get_first_deep_pair_with_neighbours(
                        depth + 1,
                        upd_left,
                        upd_pair,
                        upd_right,
                    )
                }
            },
            SNumber::Number(nr) => match (last_left, last_pair, last_right) {
                (l, Some(cv), None) => (l, Some(cv), Some(nr)),
                (_, None, _) => (Some(nr), None, None),
                (l, c, r) => (l, c, r),
            },
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            SNumber::Pair(p) => p.0.magnitude() * 3 + p.1.magnitude() * 2,
            SNumber::Number(v) => *v as usize,
        }
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &Vec<SNumber>) -> usize {
    let res = input[1..]
        .iter()
        .fold(input[0].clone(), |acc, next| acc.clone() + next.clone());
    res.magnitude()
}

//#[aoc(day, part2)]
pub fn part2(input: &Vec<SNumber>) -> usize {
    input
        .iter()
        .tuple_combinations()
        .map(|(lhs, rhs)| lhs.clone() + rhs.clone())
        .map(|v| v.magnitude())
        .max()
        .unwrap()
}
