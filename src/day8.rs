use std::{
    fmt,
    ops::{BitAnd, Deref},
};

use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day8.txt").expect("Could not find day 8 data!");
    let _testinput =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

const A: u8 = 0b00000001;
const B: u8 = 0b00000010;
const C: u8 = 0b00000100;
const D: u8 = 0b00001000;
const E: u8 = 0b00010000;
const F: u8 = 0b00100000;
const G: u8 = 0b01000000;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pattern(u8);

impl Pattern {
    fn parse(pat: &str) -> Pattern {
        let mut res = 0;
        pat.chars().for_each(|c| match c {
            'a' => res |= A,
            'b' => res |= B,
            'c' => res |= C,
            'd' => res |= D,
            'e' => res |= E,
            'f' => res |= F,
            'g' => res |= G,
            _ => {}
        });
        Pattern(res)
    }
}

impl Deref for Pattern {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:07b}", self.0)
    }
}

pub struct Observation {
    unique_patterns: [Pattern; 10],
    outputs: [Pattern; 4],
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Vec<Observation> {
    input
        .lines()
        .map(|line| {
            let (patterns, outputs) = line.split_once(" | ").unwrap();
            let pattern_vec = patterns
                .split_whitespace()
                .map(|pat| Pattern::parse(pat))
                .collect_vec();
            let output_vec = outputs
                .split_whitespace()
                .map(|pat| Pattern::parse(pat))
                .collect_vec();
            Observation {
                unique_patterns: pattern_vec.try_into().unwrap(),
                outputs: output_vec.try_into().unwrap(),
            }
        })
        .collect_vec()
}

#[derive(Debug)]
pub struct Assignment {
    assignments: [Option<Pattern>; 10],
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, ele) in self.assignments.iter().enumerate() {
            writeln!(
                f,
                "{}: {}",
                idx,
                ele.map(|v| v.to_string()).unwrap_or("".to_string())
            )?
        }
        Ok(())
    }
}

impl Assignment {
    fn new() -> Assignment {
        Assignment {
            assignments: [None; 10],
        }
    }

    fn has_value_for(&self, pattern: &Pattern) -> bool {
        self.assignments
            .iter()
            .any(|opta| opta.map(|a| a == *pattern).unwrap_or(false))
    }

    fn get_value_for(&self, pattern: &Pattern) -> usize {
        self.assignments
            .iter()
            .enumerate()
            .find_map(|(idx, opta)| -> Option<usize> {
                opta.map(|a| if a == *pattern { Some(idx) } else { None })
                    .flatten()
            })
            .unwrap_or(usize::MAX)
    }

    fn derive_easy_patterns_from(&mut self, unique_patterns: &[Pattern; 10]) {
        unique_patterns.iter().for_each(|pat| {
            if pat.count_ones() == 2 {
                self.assignments[1] = Some(*pat)
            } else if pat.count_ones() == 3 {
                self.assignments[7] = Some(*pat)
            } else if pat.count_ones() == 4 {
                self.assignments[4] = Some(*pat)
            } else if pat.count_ones() == 7 {
                self.assignments[8] = Some(*pat)
            }
        });
    }

    ///  0: 6*   1: 2*
    ///  1: 2*   7: 3*
    ///  2: 5*   4: 4*
    ///  3: 5*   2: 5
    ///  4: 4*   3: 5
    ///  5: 5*   5: 5  
    ///  6: 6*   0: 6
    ///  7: 3*   6: 6   
    ///  8: 7*   9: 6*
    ///  9: 6*   8: 7*
    fn derive_complex(&mut self, unique_patterns: &[Pattern; 10]) {
        let mut unknowns = unique_patterns.clone().to_vec();
        self.assignments.iter().flatten().for_each(|a| {
            if unknowns.contains(a) {
                unknowns = unknowns.iter().cloned().filter(|v| v != a).collect_vec()
            }
        });
        //println!(
        //    "{}",
        //    unique_patterns.iter().map(|v| v.to_string()).join(", ")
        //);
        let _one = self.assignments[1].unwrap();
        let four = self.assignments[4].unwrap();
        let seven = self.assignments[7].unwrap();
        let _eight = self.assignments[8].unwrap();

        let six = *unknowns
            .iter()
            .filter(|v| v.count_ones() == 6)
            .find(|v| seven.bitand(***v) != *seven)
            .unwrap();
        self.assignments[6] = Some(six);
        unknowns = unknowns.iter().cloned().filter(|&v| v != six).collect_vec();

        let six_lettered = unknowns
            .iter()
            .filter(|v| v.count_ones() == 6)
            .collect_vec();
        let nine = **six_lettered
            .iter()
            .find(|&v| v.bitand(*four) == *four)
            .unwrap();
        let zero = **six_lettered.iter().find(|&&&v| v != nine).unwrap();
        self.assignments[0] = Some(zero);
        self.assignments[9] = Some(nine);
        unknowns = unknowns
            .iter()
            .cloned()
            .filter(|&v| !six_lettered.contains(&&v))
            .collect_vec();

        let five = *unknowns.iter().find(|v| v.0.bitand(six.0) == v.0).unwrap();
        self.assignments[5] = Some(five);

        let three = *unknowns
            .iter()
            .find(|v| seven.0.bitand(v.0) == seven.0)
            .unwrap();
        self.assignments[3] = Some(three);

        let two = unknowns
            .iter()
            .cloned()
            .find(|v| *v != three && *v != five)
            .unwrap();
        self.assignments[2] = Some(two);

        assert!(self.assignments.iter().all(|v| v.is_some()));
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &[Observation]) -> usize {
    input
        .iter()
        .map(|observation| {
            let Observation {
                unique_patterns,
                outputs,
            } = observation;
            let mut assignment = Assignment::new();

            assignment.derive_easy_patterns_from(unique_patterns);

            outputs
                .iter()
                .filter(|val| assignment.has_value_for(*val))
                .count()
        })
        .sum()
}

//#[aoc(day, part2)]
pub fn part2(input: &[Observation]) -> u128 {
    input
        .iter()
        .map::<u128, _>(|observation| {
            let Observation {
                unique_patterns,
                outputs,
            } = observation;
            let mut assignment = Assignment::new();

            assignment.derive_easy_patterns_from(unique_patterns);
            assignment.derive_complex(unique_patterns);

            outputs
                .iter()
                .enumerate()
                .map(|(idx, val)| {
                    assignment.get_value_for(val) as u128 * 10u128.pow(3u32 - idx as u32)
                })
                .sum()
        })
        .sum()
}
