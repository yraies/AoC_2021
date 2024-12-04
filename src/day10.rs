use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day10.txt").expect("Could not find day 10 data!");
    let _testinput = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    let parsed_data = parse_data(&_testinput);
    println!("Part 1 (Test): {}", part1(&parsed_data));
    println!("Part 2 (Test): {}", part2(&parsed_data));
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum BlockType {
    Paren,
    Bracket,
    Brace,
    Chevron,
}

impl BlockType {
    fn opening(c: &char) -> Option<BlockType> {
        match c {
            '(' => Some(BlockType::Paren),
            '[' => Some(BlockType::Bracket),
            '{' => Some(BlockType::Brace),
            '<' => Some(BlockType::Chevron),
            ')' | ']' | '}' | '>' => None,
            _ => panic!("illegal opener {}", c),
        }
    }
    fn closing(c: &char) -> Option<BlockType> {
        match c {
            ')' => Some(BlockType::Paren),
            ']' => Some(BlockType::Bracket),
            '}' => Some(BlockType::Brace),
            '>' => Some(BlockType::Chevron),
            '(' | '[' | '{' | '<' => None,
            _ => panic!("illegal closer {}", c),
        }
    }
    fn syntax_checker_score(&self) -> usize {
        match self {
            BlockType::Paren => (3),
            BlockType::Bracket => (57),
            BlockType::Brace => (1197),
            BlockType::Chevron => (25137),
        }
    }
    fn autocomplete_score(&self) -> usize {
        match self {
            BlockType::Paren => (1),
            BlockType::Bracket => (2),
            BlockType::Brace => (3),
            BlockType::Chevron => (4),
        }
    }
}

#[allow(dead_code)]
pub enum ParsingErrType {
    Incomplete(Vec<BlockType>),
    Corrupted(BlockType, BlockType),
    IllegalChar(char),
    Superfluous(BlockType),
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Vec<Result<(), ParsingErrType>> {
    input
        .lines()
        .map(|line| {
            let mut stack: Vec<BlockType> = vec![];

            for c in line.trim().chars() {
                if let Some(opener) = BlockType::opening(&c) {
                    stack.push(opener);
                } else if let Some(closer) = BlockType::closing(&c) {
                    let last = stack
                        .pop()
                        .ok_or(ParsingErrType::Superfluous(closer.clone()))?;
                    if last != closer {
                        //println!(
                        //    "{} - Expected {:?}, but found {:?} instead.",
                        //    line, last, closer
                        //);
                        return Err(ParsingErrType::Corrupted(last, closer));
                    }
                }
            }

            if stack.len() == 0 {
                Ok(())
            } else {
                Err(ParsingErrType::Incomplete(stack))
            }
        })
        .collect_vec()
}

//#[aoc(day, part1)]
pub fn part1(input: &[Result<(), ParsingErrType>]) -> usize {
    input
        .iter()
        .filter_map(|v| match v {
            Err(ParsingErrType::Corrupted(_expected, got)) => Some(got.syntax_checker_score()),
            _ => None,
        })
        .sum()
}

//#[aoc(day, part2)]
pub fn part2(input: &[Result<(), ParsingErrType>]) -> usize {
    let scores = input
        .iter()
        .filter_map(|v| match v {
            Err(ParsingErrType::Incomplete(stack)) => Some(
                stack
                    .iter()
                    .rev()
                    .fold(0, |acc, next| acc * 5 + next.autocomplete_score()),
            ),
            _ => None,
        })
        .sorted()
        .collect_vec();
    scores[scores.len() / 2]
}
