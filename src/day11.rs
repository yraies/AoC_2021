use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};
#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day11.txt").expect("Could not find day 11 data!");
    let _testinput = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

#[derive(Debug, Clone)]
pub struct OceanMap {
    data: Vec<Vec<u8>>,
    flashed: Vec<Vec<bool>>,
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
        flashed: vec![vec!(false; 10); 10],
    }
}

impl OceanMap {
    pub fn size(&self) -> (i32, i32) {
        (self.data.len() as i32, self.data[0].len() as i32)
    }
    pub fn energize(&mut self, x: i32, y: i32) {
        let (x_size, y_size) = self.size();
        if x < 0 || y < 0 || x >= x_size || y >= y_size || self.flashed[x as usize][y as usize] {
            return;
        }
        self.data[x as usize][y as usize] += 1;
    }

    pub fn energize_all(&mut self) {
        for x in 0..self.data.len() {
            for y in 0..self.data.len() {
                self.energize(x as i32, y as i32);
            }
        }
    }

    pub fn flash(&mut self, x: i32, y: i32) {
        let (x_size, y_size) = self.size();
        if x < 0 || y < 0 || x >= x_size || y >= y_size || self.flashed[x as usize][y as usize] {
            return;
        }
        self.flashed[x as usize][y as usize] = true;
        if self.data[x as usize][y as usize] >= 9 {
            self.energize(x - 1, y - 1);
            self.energize(x - 1, y);
            self.energize(x - 1, y + 1);
            self.energize(x, y - 1);
            self.energize(x, y + 1);
            self.energize(x + 1, y - 1);
            self.energize(x + 1, y);
            self.energize(x + 1, y + 1);
        }
    }

    pub fn flash_all(&mut self) {
        loop {
            let flashables = (0..self.data.len())
                .cartesian_product(0..self.data.len())
                .filter(|&(x, y)| self.data[x][y] > 9 && !self.flashed[x][y])
                .collect_vec();
            //println!("{:?}", flashables);
            if flashables.len() == 0 {
                break;
            } else {
                flashables
                    .iter()
                    .for_each(|&(x, y)| self.flash(x as i32, y as i32));
            }
        }
    }

    pub fn count_flasehd(&self) -> usize {
        self.flashed.iter().flatten().filter(|&&v| v).count()
    }

    pub fn reset(&mut self) {
        for x in 0..self.data.len() {
            for y in 0..self.data.len() {
                self.flashed[x][y] = false;
                if self.data[x][y] > 9 {
                    self.data[x][y] = 0;
                }
            }
        }
    }

    pub fn synced(&self) -> bool {
        self.flashed.iter().flatten().all(|&v| v)
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &OceanMap) -> usize {
    let mut map = input.clone();
    let mut total_flahes = 0;
    //draw_map("Before step", 1, &map);
    for _i in 1..=100 {
        map.energize_all();
        map.flash_all();
        //draw_flashes(&map);
        total_flahes += map.count_flasehd();
        map.reset();
        //draw_map("After step", i, &map);
    }
    total_flahes
}

#[allow(dead_code)]
fn draw_map(name: &str, i: i32, map: &OceanMap) {
    println!(
        "{} {:3}:\n{}",
        name,
        i,
        map.data
            .iter()
            .map(|line| line.iter().map(|v| v.to_string()).join(""))
            .join("\n")
    );
}

#[allow(dead_code)]
fn draw_flashes(map: &OceanMap) {
    println!(
        "{}\n",
        map.flashed
            .iter()
            .map(|line| line.iter().map(|v| if *v { "X" } else { "." }).join(""))
            .join("\n")
    );
}

//#[aoc(day, part2)]
pub fn part2(input: &OceanMap) -> usize {
    let mut map = input.clone();
    let mut iteration = 1;
    loop {
        map.energize_all();
        map.flash_all();
        if map.synced() {
            return iteration;
        }
        map.reset();
        iteration += 1;
    }
}
