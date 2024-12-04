//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day#.txt").expect("Could not find day # data!");
    let _testinput = "
        ";
    let parsed_data = parse_data(&_testinput);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> usize {
    input.lines();
    0
}

//#[aoc(day, part1)]
pub fn part1(_input: &usize) -> usize {
    0
}

//#[aoc(day, part2)]
pub fn part2(_input: &usize) -> usize {
    0
}
