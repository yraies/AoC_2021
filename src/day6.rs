use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day6.txt").expect("Could not find day 6 data!");
    let _testinput = "3,4,3,1,2";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

#[derive(Debug, Clone)]
pub struct FishSurvey {
    ages: [u64; 9],
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> FishSurvey {
    let sorted_ages = input
        .split(",")
        .map(|nr| {
            nr.trim_end()
                .parse::<u64>()
                .expect(&format!("Error parsing :'{}'", nr))
        })
        .sorted()
        .collect_vec();
    let mut ages: [u64; 9] = [0; 9];
    for i in sorted_ages {
        ages[i as usize] += 1;
    }
    FishSurvey { ages }
}

impl FishSurvey {
    pub fn step(&mut self) {
        let new_fishes = self.ages[0];
        for i in 1..=8 {
            self.ages[i - 1] = self.ages[i];
        }
        self.ages[6] += new_fishes;
        self.ages[8] = new_fishes;
    }

    pub fn count(&self) -> u64 {
        self.ages.iter().sum()
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &FishSurvey) -> u64 {
    let mut survey = input.clone();
    for _ in 0..80 {
        survey.step()
    }
    survey.count()
}

//#[aoc(day, part2)]
pub fn part2(input: &FishSurvey) -> u64 {
    let mut survey = input.clone();
    for _ in 0..256 {
        survey.step()
    }
    survey.count()
}
