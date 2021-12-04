use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day4.txt").expect("Could not find day 4 data!");
    let _testinput = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BingoBoard {
    pub content: Vec<u64>,
    pub marks: Vec<bool>,
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> (Vec<u64>, Vec<BingoBoard>) {
    let mut lines = input.lines();
    let draw_order = lines
        .next()
        .expect("Expected draw order!")
        .split(",")
        .map(|nr| nr.parse::<u64>().expect("Expected numbers in draw order"))
        .collect_vec();
    let boards = lines
        .chunks(6)
        .into_iter()
        .map(|chunk| {
            let res = chunk
                .into_iter()
                .join(" ")
                .split_whitespace()
                .map(|nr| nr.parse::<u64>())
                .collect::<Result<Vec<_>, _>>();
            res.map_err(|err| err.to_string()).and_then(|nrs| {
                if nrs.len() != 25 {
                    Err("Invalid board size".to_string())
                } else {
                    Ok(BingoBoard::new(nrs))
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    (draw_order, boards)
}

impl BingoBoard {
    pub fn new(content: Vec<u64>) -> BingoBoard {
        let marks = vec![false; content.len()];
        BingoBoard { content, marks }
    }

    pub fn mark(&mut self, number: u64) {
        if let Some(idx) = self.content.iter().position(|c| *c == number) {
            self.marks[idx] = true;
        }
    }

    pub fn unmark(&mut self, number: u64) {
        if let Some(idx) = self.content.iter().position(|c| *c == number) {
            self.marks[idx] = false;
        }
    }

    pub fn score(&self) -> u64 {
        let mut score = 0;
        for (idx, marked) in self.marks.iter().enumerate() {
            if !marked {
                score += self.content[idx]
            }
        }
        score
    }

    pub fn wins(&self) -> bool {
        self.marks
            .chunks(5)
            .any(|chunk| chunk.iter().all(|marked| *marked))
            || self.marks.iter().skip(0).step_by(5).all(|marked| *marked)
            || self.marks.iter().skip(1).step_by(5).all(|marked| *marked)
            || self.marks.iter().skip(2).step_by(5).all(|marked| *marked)
            || self.marks.iter().skip(3).step_by(5).all(|marked| *marked)
            || self.marks.iter().skip(4).step_by(5).all(|marked| *marked)
    }
}

//#[aoc(day, part1)]
pub fn part1((draw_order, board_templates): &(Vec<u64>, Vec<BingoBoard>)) -> u64 {
    let mut boards = board_templates.iter().cloned().collect_vec();
    for next_draw in draw_order {
        for board in boards.iter_mut() {
            board.mark(*next_draw);
            if board.wins() {
                return board.score() * next_draw;
            }
        }
    }
    return u64::MAX;
}

//#[aoc(day, part2)]
pub fn part2((draw_order, board_templates): &(Vec<u64>, Vec<BingoBoard>)) -> u64 {
    let mut boards = board_templates.iter().cloned().collect_vec();
    for next_draw in draw_order {
        for board in boards.iter_mut() {
            board.mark(*next_draw);
        }
        if boards.len() == 1 && boards[0].wins() {
            return boards[0].score() * next_draw;
        }
        boards = boards
            .into_iter()
            .filter(|board| !board.wins())
            .collect_vec();
    }
    return u64::MAX;
}
