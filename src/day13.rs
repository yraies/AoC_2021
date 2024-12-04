use std::fmt;

use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day13.txt").expect("Could not find day 13 data!");
    let _testinput = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldingInstruction {
    XAxis(u32),
    YAxis(u32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrigamiPaper {
    dots: Vec<(u32, u32)>,
    instructions: Vec<FoldingInstruction>,
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> OrigamiPaper {
    let (coordinates, instructions) = input.split_once("\n\n").unwrap();
    OrigamiPaper {
        dots: coordinates
            .lines()
            .map(|line| {
                let (xstr, ystr) = line.split_once(",").unwrap();
                (xstr.parse::<u32>().unwrap(), ystr.parse::<u32>().unwrap())
            })
            .collect(),
        instructions: instructions
            .lines()
            .map(|line| {
                if line.contains("x") {
                    FoldingInstruction::XAxis(
                        line.strip_prefix("fold along x=")
                            .map(|nr| nr.parse::<u32>().unwrap())
                            .unwrap(),
                    )
                } else {
                    FoldingInstruction::YAxis(
                        line.strip_prefix("fold along y=")
                            .map(|nr| nr.parse::<u32>().unwrap())
                            .unwrap(),
                    )
                }
            })
            .collect_vec(),
    }
}

impl OrigamiPaper {
    fn fold_next(&mut self) {
        let fold = self.instructions.first().unwrap();

        let new_dots = self
            .dots
            .iter()
            .map(|(x, y)| -> (u32, u32) {
                match fold {
                    FoldingInstruction::XAxis(dx) => (if x > dx { 2 * dx - x } else { *x }, *y),
                    FoldingInstruction::YAxis(dy) => (*x, if y > dy { 2 * dy - y } else { *y }),
                }
            })
            .sorted()
            .dedup()
            .collect_vec();

        self.dots = new_dots;

        self.instructions.remove(0);
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &OrigamiPaper) -> usize {
    let mut paper = input.clone();
    paper.fold_next();
    paper.dots.len()
}

impl fmt::Display for OrigamiPaper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let maxx = *self.dots.iter().map(|(x, _)| x).max().unwrap();
        let maxy = *self.dots.iter().map(|(_, y)| y).max().unwrap();
        for y in 0..=maxy {
            for x in 0..=maxx {
                write!(
                    f,
                    "{}",
                    if self.dots.contains(&(x, y)) {
                        "#"
                    } else {
                        " "
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

//#[aoc(day, part2)]
pub fn part2(input: &OrigamiPaper) -> String {
    let mut paper = input.clone();
    while paper.instructions.len() > 0 {
        paper.fold_next();
    }
    format!("\n{}", paper)
}
