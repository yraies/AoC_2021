//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let input = std::fs::read_to_string("input/day3.txt").expect("Could not find day 3 data!");
    let _testinput = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";
    let parsed_data = parse_data(&input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> (usize, Vec<u64>) {
    let len = input.lines().next().unwrap().len();
    (
        len,
        input
            .lines()
            .map(|line| u64::from_str_radix(line, 2).expect("Not a binary number!"))
            .collect(),
    )
}

//#[aoc(day, part1)]
pub fn part1((digits, input): &(usize, Vec<u64>)) -> u64 {
    let gamma = most_common(*digits, input);
    let epsilon = least_common(*digits, input);

    gamma * epsilon
}

fn most_common(digits: usize, input: &Vec<u64>) -> u64 {
    count_digits_with(|ones, zeros| ones >= zeros, digits, input)
}
fn least_common(digits: usize, input: &Vec<u64>) -> u64 {
    count_digits_with(|ones, zeros| ones < zeros, digits, input)
}

fn count_digits_with(comp_fun: fn(usize, usize) -> bool, digits: usize, input: &Vec<u64>) -> u64 {
    (0..digits)
        .map(|idx| {
            let ones = input.iter().filter(|nr| ((*nr >> idx) & 0b1) > 0).count();
            let zeros = input.iter().filter(|nr| ((*nr >> idx) & 0b1) == 0).count();

            if comp_fun(ones, zeros) {
                1 << idx
            } else {
                0
            }
        })
        .sum()
}

//#[aoc(day, part2)]
pub fn part2((digits, input): &(usize, Vec<u64>)) -> u64 {
    let oxygen = get_rating(*digits, input, most_common);
    let co2 = get_rating(*digits, input, least_common);

    oxygen * co2
}

fn get_rating(digits: usize, input: &Vec<u64>, bit_crit: fn(usize, &Vec<u64>) -> u64) -> u64 {
    let mut values = input.iter().cloned().collect::<Vec<u64>>();

    for idx in (0..digits).rev() {
        let mask = 1 << idx;
        let match_val = bit_crit(digits, &values) & mask;

        values = values
            .into_iter()
            .filter(|nr| (*nr & mask) == match_val)
            .collect();

        if values.len() == 1 {
            break;
        }
    }

    values[0]
}
