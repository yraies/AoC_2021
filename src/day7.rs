use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day7.txt").expect("Could not find day 7 data!");
    let _testinput = "16,1,2,0,4,2,7,1,2,14";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Vec<usize> {
    input
        .split(",")
        .map(|nr| nr.trim_end().parse::<usize>().unwrap())
        .collect_vec()
}

fn find_min_fuel(
    input: &[usize],
    min_pos: usize,
    max_pos: usize,
    cost_by_distance: fn(usize) -> usize,
) -> usize {
    (min_pos..=max_pos)
        .map(|target| {
            (
                target,
                input
                    .iter()
                    .map(|&pos| {
                        let n = (target as i64 - pos as i64).abs() as usize;
                        cost_by_distance(n)
                    })
                    .sum::<usize>(),
            )
        })
        .min_by_key(|(_, fuel)| *fuel)
        .unwrap()
        .1
}

//#[aoc(day, part1)]
pub fn part1(input: &[usize]) -> usize {
    let (&min_pos, &max_pos) = input.iter().minmax().into_option().unwrap();
    find_min_fuel(input, min_pos, max_pos, |n| n)
}

//#[aoc(day, part2)]
pub fn part2(input: &[usize]) -> usize {
    let (&min_pos, &max_pos) = input.iter().minmax().into_option().unwrap();
    find_min_fuel(input, min_pos, max_pos, |n| (n * n + n) / 2)
}
