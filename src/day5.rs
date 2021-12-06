use std::collections::HashMap;

use itertools::Itertools;
//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day5.txt").expect("Could not find day 5 data!");
    let _testinput = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

#[derive(Debug)]
pub struct LineSegment {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Vec<LineSegment> {
    input
        .lines()
        .map(|line| {
            let nrs = line
                .split(" -> ")
                .flat_map(|part| part.split(","))
                .map(|nr| nr.parse::<u32>().unwrap())
                .collect_vec();
            LineSegment {
                x1: nrs[0],
                y1: nrs[1],
                x2: nrs[2],
                y2: nrs[3],
            }
        })
        .collect_vec()
}

fn sort_tuple((a, b): (u32, u32)) -> (u32, u32) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &Vec<LineSegment>) -> usize {
    let mut smog_map = HashMap::<(u32, u32), usize>::new();
    input.iter().for_each(|line| {
        let (x1, x2) = sort_tuple((line.x1, line.x2));
        let (y1, y2) = sort_tuple((line.y1, line.y2));

        if x1 == x2 {
            (y1..=y2).for_each(|y| {
                let counter = *smog_map.get(&(x1, y)).unwrap_or(&0);
                smog_map.insert((x1, y), counter + 1);
            })
        } else if y1 == y2 {
            (x1..=x2).for_each(|x| {
                let counter = *smog_map.get(&(x, y1)).unwrap_or(&0);
                smog_map.insert((x, y1), counter + 1);
            })
        }
    });

    smog_map
        .iter()
        .filter(|(_pos, &counter)| counter >= 2)
        .count()
}

//#[aoc(day, part2)]
pub fn part2(input: &Vec<LineSegment>) -> usize {
    let mut smog_map = HashMap::<(u32, u32), usize>::new();
    input.iter().for_each(|line| {
        let (x1, x2) = sort_tuple((line.x1, line.x2));
        let (y1, y2) = sort_tuple((line.y1, line.y2));

        if x1 == x2 || y1 == y2 || x2 - x1 == y2 - y1 {
            let mut xs = if line.x1 > line.x2 {
                (line.x2..=line.x1).collect_vec()
            } else {
                (line.x1..=line.x2).rev().collect_vec()
            };
            let mut ys = if line.y1 > line.y2 {
                (line.y2..=line.y1).collect_vec()
            } else {
                (line.y1..=line.y2).rev().collect_vec()
            };

            if xs.len() == 1 {
                xs.resize(ys.len(), xs[0])
            };
            if ys.len() == 1 {
                ys.resize(xs.len(), ys[0])
            };

            xs.iter().zip(ys.iter()).for_each(|(&x, &y)| {
                let counter = *smog_map.get(&(x, y)).unwrap_or(&0);
                smog_map.insert((x, y), counter + 1);
            })
        }
    });

    smog_map
        .iter()
        .filter(|(_pos, &counter)| counter >= 2)
        .count()
}
