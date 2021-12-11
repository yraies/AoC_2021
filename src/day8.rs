use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day8.txt").expect("Could not find day 8 data!");
    let _testinput = "";
    let parsed_data = parse_data(&_testinput);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> usize {
    input.lines().map(|line| {
        let (patterns, outputs) = line.split_once(" | ").unwrap();
        let pattern_vec = patterns.split_whitespace().collect_vec();
        let output_vec = outputs.split_whitespace().collect_vec();
    });
    0
}

//#[aoc(day, part1)]
pub fn part1(input: &usize) -> usize {
    *input
}

//#[aoc(day, part2)]
pub fn part2(input: &usize) -> usize {
    *input
}
