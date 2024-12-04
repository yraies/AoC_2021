//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day17.txt").expect("Could not find day 17 data!");
    let _testinput = "target area: x=20..30, y=-10..-5";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Rect {
    let (prex, ypart) = input.split_once(", y=").unwrap();
    let xpart = prex.strip_prefix("target area: x=").unwrap();
    let (xmin, xmax) = xpart
        .split_once("..")
        .map(|(min, max)| (min.parse::<i64>().unwrap(), max.parse::<i64>().unwrap()))
        .unwrap();
    let (ymin, ymax) = ypart
        .trim()
        .split_once("..")
        .map(|(min, max)| (min.parse::<i64>().unwrap(), max.parse::<i64>().unwrap()))
        .unwrap();
    Rect {
        xmin,
        ymin,
        xmax,
        ymax,
    }
}

#[derive(Debug)]
pub struct Rect {
    xmin: i64,
    ymin: i64,
    xmax: i64,
    ymax: i64,
}

impl Rect {
    fn contains(&self, (x, y): (i64, i64)) -> bool {
        x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax
    }

    fn reverse_xbounds(&self) -> (i64, i64) {
        fn reverse_gauss(a: f64) -> i64 {
            (((f64::sqrt(8f64 * a.abs() + 1f64) - 1f64) / 2f64).floor() * a.signum()) as i64
        }
        let min_bound = reverse_gauss(self.xmin as f64);
        let max_bound = reverse_gauss(self.xmax as f64) + 1;

        (min_bound, max_bound)
    }

    fn reverse_ybounds(&self, _dx: i64) -> (i64, i64) {
        let ydiff = self.ymax - self.ymin;
        (-ydiff, ydiff * ydiff)
    }
}

#[derive(Debug, Clone)]
pub struct Probe {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Probe {
    fn step(&mut self) {
        if false {
            println!(
                "Stepping: x: {}+={}, y: {}+={}",
                self.x, self.dx, self.y, self.dy
            );
        }
        self.x += self.dx;
        self.y += self.dy;
        self.dx -= self.dx.signum();
        self.dy -= 1;
    }

    fn lands_in(&self, target_area: &Rect) -> bool {
        let mut sim_clone = self.clone();
        while sim_clone.dy > 0 || sim_clone.y >= target_area.ymin {
            sim_clone.step();
            if target_area.contains((sim_clone.x, sim_clone.y)) {
                return true;
            }
        }
        return false;
    }

    fn highest_y(&self) -> i64 {
        let mut sim_clone = self.clone();
        let mut highest_y = i64::MIN;
        while sim_clone.dy > 0 {
            sim_clone.step();
            if sim_clone.y > highest_y {
                highest_y = sim_clone.y
            }
        }
        highest_y
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &Rect) -> i64 {
    let xbound = input.reverse_xbounds();

    let mut highest_y = i64::MIN;
    for start_dx in xbound.0..xbound.1 {
        let ybound = input.reverse_ybounds(start_dx);
        for start_dy in ybound.0..ybound.1 {
            //println!("Checking x={:?} and y={:?}", start_dx, start_dy);
            let probe = Probe {
                x: 0,
                y: 0,
                dx: start_dx,
                dy: start_dy,
            };
            if probe.lands_in(input) {
                let new_height = probe.highest_y();
                //println!("Found {:?} with height {}", probe, new_height);
                if new_height > highest_y {
                    highest_y = new_height;
                }
            }
        }
    }

    highest_y
}

//#[aoc(day, part2)]
pub fn part2(input: &Rect) -> i64 {
    let xbound = input.reverse_xbounds();

    let mut ctr = 0;
    for start_dx in xbound.0..xbound.1.pow(2) {
        if start_dx % 10 == 0 {
            println!("{} of {}-{}", start_dx, xbound.0, xbound.1.pow(2));
        }
        let ybound = input.reverse_ybounds(start_dx);
        for start_dy in (-ybound.1)..ybound.1 {
            let probe = Probe {
                x: 0,
                y: 0,
                dx: start_dx,
                dy: start_dy,
            };
            if probe.lands_in(input) {
                //print!("{},{}\t", start_dx, start_dy);
                ctr += 1;
            }
        }
    }

    ctr
}
