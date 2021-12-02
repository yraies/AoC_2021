use itertools::Itertools;

#[main]
#[allow(dead_code)]
pub fn main() {
    let input = std::fs::read_to_string("input/day1.txt").expect("Could not find day 1 data!");
    let _testinput = "119
200
208
210
200
207
240
269
260
263";
    let parsed_data = parse_data(&input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

pub fn parse_data(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

pub fn part1(input: &[usize]) -> usize {
    input
        .iter()
        .fold((usize::max_value(), 0), |(last, count), next| {
            (*next, if *next > last { count + 1 } else { count })
        })
        .1
}

pub fn part2(input: &[usize]) -> usize {
    input
        .iter()
        .tuple_windows::<(_, _, _)>()
        .fold((usize::max_value(), 0), |(last, count), tup| {
            let next = tup.0 + tup.1 + tup.2;
            (next, if next > last { count + 1 } else { count })
        })
        .1
}
