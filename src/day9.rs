use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day9.txt").expect("Could not find day 9 data!");
    let _testinput = "2199943210
3987894921
9856789892
8767896789
9899965678";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

pub struct OceanMap {
    data: Vec<Vec<u8>>,
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> OceanMap {
    OceanMap {
        data: input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect_vec()
            })
            .collect_vec(),
    }
}

impl OceanMap {
    pub fn is_low_point(&self, x: i32, y: i32) -> bool {
        let self_height = self.height(x, y);
        self.height(x - 1, y) > self_height
            && self.height(x + 1, y) > self_height
            && self.height(x, y - 1) > self_height
            && self.height(x, y + 1) > self_height
    }

    pub fn risk_level(&self, x: i32, y: i32) -> u8 {
        self.height(x, y) + 1
    }

    pub fn height(&self, x: i32, y: i32) -> u8 {
        let (x_size, y_size) = self.size();
        if x < 0 || y < 0 || x >= x_size || y >= y_size {
            u8::MAX
        } else {
            self.data[x as usize][y as usize]
        }
    }

    pub fn size(&self) -> (i32, i32) {
        (self.data.len() as i32, self.data[0].len() as i32)
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &OceanMap) -> usize {
    let (x_size, y_size) = input.size();
    (0..(x_size))
        .cartesian_product(0..y_size)
        .filter(|&(x, y)| input.is_low_point(x, y))
        .map(|(x, y)| input.risk_level(x, y) as usize)
        .sum()
}

//#[aoc(day, part2)]
pub fn part2(input: &OceanMap) -> usize {
    let (x_size, y_size) = {
        let (x, y) = input.size();
        (x as usize, y as usize)
    };
    let mut basin_map = vec![vec!(0; y_size); x_size];
    (0..(x_size))
        .cartesian_product(0..y_size)
        .filter(|&(x, y)| input.is_low_point(x as i32, y as i32))
        .enumerate()
        .for_each(|(idx, (x, y))| basin_map[x][y] = idx + 1);
    (0..(x_size))
        .cartesian_product(0..y_size)
        .filter(|&(x, y)| input.height(x as i32, y as i32) == 9)
        .for_each(|(x, y)| basin_map[x][y] = usize::MAX);

    fn wash(array: &mut Vec<Vec<usize>>) {
        let (x_size, y_size) = { (array.len() as usize, array[0].len() as usize) };

        for x in 0..x_size {
            let mut brush = 0;
            for y in 0..y_size {
                if array[x][y] == usize::MAX {
                    brush = 0;
                } else if array[x][y] == 0 {
                    array[x][y] = brush;
                } else {
                    brush = array[x][y];
                }
            }
            for y in (0..y_size).rev() {
                if array[x][y] == usize::MAX {
                    brush = 0;
                } else if array[x][y] == 0 {
                    array[x][y] = brush;
                } else {
                    brush = array[x][y];
                }
            }
        }
        for y in 0..y_size {
            let mut brush = 0;
            for x in 0..x_size {
                if array[x][y] == usize::MAX {
                    brush = 0;
                } else if array[x][y] == 0 {
                    array[x][y] = brush;
                } else {
                    brush = array[x][y];
                }
            }
            for x in (0..x_size).rev() {
                if array[x][y] == usize::MAX {
                    brush = 0;
                } else if array[x][y] == 0 {
                    array[x][y] = brush;
                } else {
                    brush = array[x][y];
                }
            }
        }
    }

    for _ in 0..10 {
        wash(&mut basin_map);
    }

    //println!(
    //    "{}",
    //    basin_map
    //        .iter()
    //        .map(|line| line
    //            .iter()
    //            .map(|v| if *v < 10 {
    //                v.to_string()
    //            } else {
    //                "X".to_string()
    //            })
    //            .join(""))
    //        .join("\n")
    //);
    let basin_sizes = basin_map
        .into_iter()
        .flatten()
        .filter(|&v| v != usize::MAX)
        .sorted()
        .fold(vec![0], |mut acc, next| {
            let len = acc.len();
            if next == len {
                acc[len - 1] += 1;
                acc
            } else {
                acc.push(1);
                acc
            }
        });

    basin_sizes
        .iter()
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc, next| acc * next)
}
