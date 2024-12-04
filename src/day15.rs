use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day15.txt").expect("Could not find day 15 data!");
    let _testinput = "
        ";
    let parsed_data = parse_data(&_testinput);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect_vec())
        .collect_vec()
}

//#[aoc(day, part1)]
pub fn part1(_input: &Vec<Vec<u32>>) -> usize {
    0
}

//#[aoc(day, part2)]
pub fn part2(_input: &Vec<Vec<u32>>) -> usize {
    0
}
