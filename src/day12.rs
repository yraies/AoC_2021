use std::collections::HashMap;

use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() {
    let _input = std::fs::read_to_string("input/day12.txt").expect("Could not find day 12 data!");
    let _testinput = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    let parsed_data = parse_data(&_input);
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
}

pub struct Graph {
    _name_by_id: HashMap<usize, String>,
    id_by_name: HashMap<String, usize>,
    adjacencies: Vec<Vec<usize>>,
    is_large: Vec<bool>,
}

impl Graph {
    fn new_from_names(names: &[String]) -> Graph {
        let mut _name_by_id = HashMap::new();
        let mut id_by_name = HashMap::new();
        let mut is_large = vec![];
        let mut curr_id = 0;
        for name in names {
            id_by_name.insert(name.to_owned(), curr_id);
            _name_by_id.insert(curr_id, name.to_owned());
            is_large.push(name.eq(&name.to_uppercase()));
            curr_id += 1;
        }
        Graph {
            _name_by_id,
            id_by_name,
            is_large,
            adjacencies: vec![vec!(); names.len()],
        }
    }

    fn add_edge(&mut self, a: String, b: String) {
        let a_id = *self.id_by_name.get(&a).unwrap();
        let b_id = *self.id_by_name.get(&b).unwrap();
        let a_adj = &mut self.adjacencies[a_id];
        if !a_adj.contains(&b_id) {
            a_adj.push(b_id);
        }
        let b_adj = &mut self.adjacencies[b_id];
        if !b_adj.contains(&a_id) {
            b_adj.push(a_id);
        }
    }

    fn find_all_from(&self, start: &String, end: &String) -> Vec<Vec<usize>> {
        self.find_rec(
            *self.id_by_name.get(end).unwrap(),
            &vec![*self.id_by_name.get(start).unwrap()],
        )
    }

    fn find_rec(&self, end: usize, path: &[usize]) -> Vec<Vec<usize>> {
        let last_node = *path.last().unwrap();
        if last_node == end {
            return vec![path.to_vec()];
        }
        let neighbours = &self.adjacencies[last_node];
        neighbours
            .iter()
            .filter(|&next| self.is_large[*next] || !path.contains(next))
            .flat_map(|&next| {
                let mut new_path = path.to_vec();
                new_path.push(next);
                self.find_rec(end, &new_path)
            })
            .collect_vec()
    }

    fn find_all_from_special(&self, start: &String, end: &String) -> Vec<Vec<usize>> {
        self.find_rec_special(
            *self.id_by_name.get(start).unwrap(),
            *self.id_by_name.get(end).unwrap(),
            &vec![*self.id_by_name.get(start).unwrap()],
            false,
        )
    }

    fn find_rec_special(
        &self,
        start: usize,
        end: usize,
        path: &[usize],
        joker: bool,
    ) -> Vec<Vec<usize>> {
        let last_node = *path.last().unwrap();
        if last_node == end {
            return vec![path.to_vec()];
        }
        let neighbours = &self.adjacencies[last_node];
        neighbours
            .iter()
            .filter_map(|&next| {
                if next == start {
                    None
                } else if self.is_large[next] {
                    return Some((next, joker));
                } else if !path.contains(&next) {
                    return Some((next, joker));
                } else if !joker {
                    return Some((next, true));
                } else {
                    None
                }
            })
            .flat_map(|(next, new_joker)| {
                let mut new_path = path.to_vec();
                new_path.push(next);
                self.find_rec_special(start, end, &new_path, new_joker)
            })
            .collect_vec()
    }
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Graph {
    let names = input
        .lines()
        .map(|line| line.split("-").map(|s| s.to_string()))
        .flatten()
        .sorted()
        .dedup()
        .collect_vec();
    let mut graph = Graph::new_from_names(&names);
    input.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        graph.add_edge(a.to_string(), b.to_string());
    });
    graph
}

//#[aoc(day, part1)]
pub fn part1(input: &Graph) -> usize {
    input
        .find_all_from(&"start".to_string(), &"end".to_string())
        .len()
}

//#[aoc(day, part2)]
pub fn part2(input: &Graph) -> usize {
    input
        .find_all_from_special(&"start".to_string(), &"end".to_string())
        .len()
}
