use std::collections::HashMap;

use itertools::Itertools;

//use aoc_runner_derive::{aoc, aoc_generator};

#[main]
#[allow(dead_code)]
pub fn main() -> Result<(), String> {
    let _input = std::fs::read_to_string("input/day14.txt").expect("Could not find day 14 data!");
    let _testinput = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
    let parsed_data = parse_data(&_input)?;
    println!("Part 1: {}", part1(&parsed_data));
    println!("Part 2: {}", part2(&parsed_data));
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    body: [char; 2],
    head: char,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grammar {
    template: Vec<char>,
    rules: Vec<Rule>,
}

//#[aoc_generator(day)]
pub fn parse_data(input: &str) -> Result<Grammar, String> {
    let (template_str, rules_str) = input
        .split_once("\n\n")
        .ok_or("Broken Formating! No double \\n")?;

    let template = template_str.trim().chars().collect_vec();
    let rules = rules_str
        .lines()
        .map(|line| {
            line.split_once(" -> ")
                .ok_or(format!("Malformed rule: {}", line))
                .and_then(|(body_str, head_str)| {
                    let body = body_str
                        .chars()
                        .collect_vec()
                        .try_into()
                        .map_err(|e| format!("Unexpected Body Length: {:?}", e))?;
                    let head = head_str
                        .chars()
                        .nth(0)
                        .ok_or(format!("Malformed rule head: {}", head_str))?;
                    Ok(Rule { body, head })
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Grammar { template, rules })
}

impl Grammar {
    fn step(&mut self) {
        let first_elem = self.template[0];
        self.template = self
            .template
            .iter()
            .tuple_windows::<(_, _)>()
            .flat_map(|(&last, &next)| {
                if let Some(rule) = self
                    .rules
                    .iter()
                    .find(|rule| rule.body[0] == last && rule.body[1] == next)
                {
                    vec![rule.head, next]
                } else {
                    vec![next]
                }
            })
            .collect_vec();
        let mut new_template = vec![first_elem];
        new_template.append(&mut self.template);
        self.template = new_template;
    }
}

//#[aoc(day, part1)]
pub fn part1(input: &Grammar) -> usize {
    let mut grammar = input.clone();
    for _ in 0..10 {
        grammar.step();
    }
    let counts = grammar
        .template
        .iter()
        .sorted()
        .dedup_with_count()
        .sorted_by_key(|v| v.0)
        .collect_vec();

    counts.last().unwrap().0 - counts.first().unwrap().0
}

//      ^ Nice part ^
//
// #######################
//
//   v Disgusting Part v

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PseudoRule {
    body: [char; 2],
    head: [[char; 2]; 2],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PseudoGrammar {
    template: HashMap<[char; 2], u128>,
    additions: HashMap<[char; 2], i128>,
    rules: Vec<PseudoRule>,
}

impl From<Rule> for PseudoRule {
    fn from(rule: Rule) -> Self {
        PseudoRule {
            body: rule.body,
            head: [[rule.body[0], rule.head], [rule.head, rule.body[1]]],
        }
    }
}

impl From<Grammar> for PseudoGrammar {
    fn from(grammar: Grammar) -> Self {
        let Grammar {
            template: old_template,
            rules: old_rules,
        } = grammar;

        let mut template = old_template
            .windows(2)
            .sorted()
            .dedup_with_count()
            .map(|(count, value)| {
                (
                    value.try_into().expect("this should not happen!"),
                    count as u128,
                )
            })
            .collect::<HashMap<[char; 2], u128>>();
        let rules = old_rules.into_iter().map(PseudoRule::from).collect_vec();
        let keys = template.keys().cloned().collect_vec();
        for rule in rules.iter().filter(|rule| !keys.contains(&&rule.body)) {
            template.insert(rule.body, 0);
        }
        let keys = template.keys().cloned().collect_vec();
        for rule in rules.iter().filter(|rule| !keys.contains(&rule.head[0])) {
            template.insert(rule.head[0], 0);
        }
        let keys = template.keys().cloned().collect_vec();
        for rule in rules.iter().filter(|rule| !keys.contains(&rule.head[1])) {
            template.insert(rule.head[1], 0);
        }
        let additions = template
            .keys()
            .map(|key: &[char; 2]| (key.to_owned(), 0))
            .collect();
        PseudoGrammar {
            template,
            rules,
            additions,
        }
    }
}

impl PseudoGrammar {
    fn step(&mut self) {
        //println!("     {}", PseudoGrammar::tostr(&self.template));
        for rule in self.rules.iter() {
            let PseudoRule { body, head } = rule;
            if let Some(&occs) = self.template.get(body) {
                *self.additions.get_mut(body).unwrap() -= occs as i128;
                *self.additions.get_mut(&head[0]).unwrap() += occs as i128;
                *self.additions.get_mut(&head[1]).unwrap() += occs as i128;
            }
        }
        //println!("ADD: {}", PseudoGrammar::tostr2(&self.additions));
        for (pair, adds) in self.additions.iter_mut() {
            let ctr = self.template.get_mut(pair).unwrap();
            *ctr = (i128::try_from(*ctr).unwrap() + *adds) as u128;
            *adds = 0;
        }
        //println!("SUM: {}\n", PseudoGrammar::tostr(&self.template));
    }

    // Debug Party ...
    //fn tostr(map: &HashMap<[char; 2], u128>) -> String {
    //    map.iter()
    //        .map(|(k, v)| format!("{}{} <- {}", k[0], k[1], v))
    //        .sorted()
    //        .join(", ")
    //}

    //fn tostr2(map: &HashMap<[char; 2], i128>) -> String {
    //    map.iter()
    //        .map(|(k, v)| format!("{}{} <- {}", k[0], k[1], v))
    //        .sorted()
    //        .join(", ")
    //}
}

//#[aoc(day, part2)]
pub fn part2(input: &Grammar) -> u128 {
    let mut grammar = PseudoGrammar::from(input.clone());

    for i in 1..=40 {
        println!(
            "Iteration {} - Polymer length {}",
            i,
            grammar.template.len()
        );
        grammar.step();
    }

    let counts = grammar
        .template
        .iter()
        .map(|(k, v)| (k[1], v))
        .sorted()
        .into_group_map_by(|(k, _occs)| *k)
        .into_iter()
        .map(|ks| (ks.0, ks.1.iter().map(|k| k.1).sum()))
        .sorted_by_key(|v: &(_, u128)| v.1)
        .collect_vec();

    println!("{:?}", counts);

    counts.last().unwrap().1 - counts.first().unwrap().1
}
