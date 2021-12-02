#[main]
#[allow(dead_code)]
pub fn main() {
    let input = std::fs::read_to_string("input/day2.txt").expect("Could not find day 2 data!");
    let _testinput = "forward 5
down 5
forward 8
up 3
down 8
forward 2";
    let parsed_data = parse_data(&input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

pub enum SubmarineCommand {
    Forawrd(i64),
    Down(i64),
    Up(i64),
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Vec<SubmarineCommand> {
    input
        .lines()
        .map(|line| {
            let split = line.split(' ').collect::<Vec<&str>>();
            match split[0] {
                "forward" => SubmarineCommand::Forawrd(split[1].parse::<i64>().unwrap()),
                "up" => SubmarineCommand::Up(split[1].parse::<i64>().unwrap()),
                "down" => SubmarineCommand::Down(split[1].parse::<i64>().unwrap()),
                _ => panic!("Invalid data!"),
            }
        })
        .collect()
}

//#[aoc(day, part1)]
pub fn part1(input: &[SubmarineCommand]) -> i64 {
    let (pos, depth) = input
        .iter()
        .fold((0i64, 0i64), |(pos, depth), next| match next {
            SubmarineCommand::Forawrd(val) => (pos + *val, depth),
            SubmarineCommand::Down(val) => (pos, depth + *val),
            SubmarineCommand::Up(val) => (pos, depth - *val),
        });
    pos * depth
}

//#[aoc(day, part2)]
pub fn part2(input: &[SubmarineCommand]) -> i64 {
    let (pos, depth, _aim) = input
        .iter()
        .fold((0i64, 0i64, 0i64), |(pos, depth, aim), next| match next {
            SubmarineCommand::Forawrd(val) => (pos + *val, depth + aim * *val, aim),
            SubmarineCommand::Down(val) => (pos, depth, aim + *val),
            SubmarineCommand::Up(val) => (pos, depth, aim - *val),
        });
    pos * depth
}
