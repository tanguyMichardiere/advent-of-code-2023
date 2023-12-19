use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::{self, Range};
use std::str::FromStr;

use crate::regex;

struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl ops::Index<&Category> for Part {
    type Output = usize;

    fn index(&self, index: &Category) -> &Self::Output {
        match index {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone)]
struct PartRange {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl ops::Index<&Category> for PartRange {
    type Output = Range<usize>;

    fn index(&self, index: &Category) -> &Self::Output {
        match index {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }
}

impl ops::IndexMut<&Category> for PartRange {
    fn index_mut(&mut self, index: &Category) -> &mut Self::Output {
        match index {
            Category::X => &mut self.x,
            Category::M => &mut self.m,
            Category::A => &mut self.a,
            Category::S => &mut self.s,
        }
    }
}

impl PartRange {
    fn split(&mut self, category: &Category, compare: &Compare, compare_to: usize) -> Self {
        let mut new_part = self.clone();
        match compare {
            Compare::Inf => {
                self[category].start = compare_to;
                new_part[category].end = compare_to;
            }
            Compare::Sup => {
                self[category].end = compare_to + 1;
                new_part[category].start = compare_to + 1;
            }
        }
        new_part
    }

    fn count(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

enum Category {
    X,
    M,
    A,
    S,
}

impl FromStr for Category {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

enum Compare {
    Inf,
    Sup,
}

impl FromStr for Compare {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Self::Inf),
            ">" => Ok(Self::Sup),
            _ => Err(()),
        }
    }
}

struct IfRule {
    category: Category,
    compare: Compare,
    compare_to: usize,
    destination: String,
}

impl FromStr for IfRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = regex!(
            r"(?P<category>[xmas])(?P<compare>[<>])(?P<compare_to>\d+):(?P<destination>[a-z]+|[AR])"
        )
        .captures(s)
        .unwrap();
        Ok(Self {
            category: caps["category"].parse().unwrap(),
            compare: caps["compare"].parse().unwrap(),
            compare_to: caps["compare_to"].parse().unwrap(),
            destination: caps["destination"].to_owned(),
        })
    }
}

impl IfRule {
    fn apply(&self, part: &Part) -> Option<String> {
        match (part[&self.category].cmp(&self.compare_to), &self.compare) {
            (Ordering::Less, Compare::Inf) | (Ordering::Greater, Compare::Sup) => {
                Some(self.destination.clone())
            }
            _ => None,
        }
    }

    fn apply_range(&self, part: &mut PartRange) -> Option<(PartRange, String)> {
        if part[&self.category].contains(&self.compare_to) {
            Some((
                part.split(&self.category, &self.compare, self.compare_to),
                self.destination.clone(),
            ))
        } else {
            None
        }
    }
}

struct Rules {
    if_rules: Vec<IfRule>,
    else_rule: String,
}

impl FromStr for Rules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            if_rules: regex!(r"(?P<if_rule>[xmas][<>]\d+:(?:[a-z]+|[AR])),")
                .captures_iter(s)
                .map(|caps| caps["if_rule"].parse().unwrap())
                .collect(),
            else_rule: regex!(r"(?P<else_rule>(?:[a-z]+|[AR]))$")
                .captures(s)
                .unwrap()["else_rule"]
                .to_owned(),
        })
    }
}

impl Rules {
    fn apply(&self, part: &Part) -> String {
        for if_rule in &self.if_rules {
            if let Some(destination) = if_rule.apply(part) {
                return destination;
            }
        }
        self.else_rule.clone()
    }

    fn apply_range(&self, mut part: PartRange) -> Vec<(PartRange, String)> {
        let mut parts = Vec::new();
        for if_rule in &self.if_rules {
            if let Some((part, destination)) = if_rule.apply_range(&mut part) {
                parts.push((part, destination));
            }
        }
        parts.push((part, self.else_rule.clone()));
        parts
    }
}

fn parse(input: &str) -> (HashMap<String, Rules>, Vec<Part>) {
    let caps = regex!(
        r"(?P<workflows>(?:[a-z]+\{(?:[xmas][<>]\d+:(?:[a-z]+|[AR]),)+(?:[a-z]+|[AR])\}\n)+)\n(?P<parts>(?:\{x=\d+,m=\d+,a=\d+,s=\d+\}\n)+)"
    ).captures(input).unwrap();
    (
        regex!(
            r"(?P<name>[a-z]+)\{(?P<rules>(?:[xmas][<>]\d+:(?:[a-z]+|[AR]),)+(?:[a-z]+|[AR]))\}"
        )
        .captures_iter(&caps["workflows"])
        .map(|caps| (caps["name"].to_owned(), caps["rules"].parse().unwrap()))
        .collect::<HashMap<String, Rules>>(),
        regex!(r"\{x=(?P<x>\d+),m=(?P<m>\d+),a=(?P<a>\d+),s=(?P<s>\d+)\}")
            .captures_iter(&caps["parts"])
            .map(|caps| Part {
                x: caps["x"].parse().unwrap(),
                m: caps["m"].parse().unwrap(),
                a: caps["a"].parse().unwrap(),
                s: caps["s"].parse().unwrap(),
            })
            .collect(),
    )
}

fn process(part: &Part, workflows: &HashMap<String, Rules>) -> bool {
    let mut current_workflow = "in".to_owned();
    loop {
        let rules = &workflows[&current_workflow];
        let next_workflow = rules.apply(part);
        match next_workflow.as_str() {
            "A" => break true,
            "R" => break false,
            _ => {
                current_workflow = next_workflow;
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let (workflows, parts) = parse(input);
    parts
        .into_iter()
        .filter(|part| process(part, &workflows))
        .map(|part| part.sum())
        .sum()
}

fn count_all(workflows: &HashMap<String, Rules>) -> usize {
    let mut parts = vec![(
        PartRange {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        },
        "in".to_owned(),
    )];
    let mut accepted = 0;
    while !parts.is_empty() {
        parts = parts
            .into_iter()
            .flat_map(|(part, workflow)| workflows[&workflow].apply_range(part))
            .filter(|(part, workflow)| match workflow.as_str() {
                "A" => {
                    accepted += part.count();
                    false
                }
                "R" => false,
                _ => true,
            })
            .collect();
    }
    accepted
}

pub fn part_two(input: &str) -> usize {
    let (workflows, _) = parse(input);
    count_all(&workflows)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/19/1").unwrap();
        assert_eq!(part_one(&input), 19114);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/19/1").unwrap();
        assert_eq!(part_two(&input), 167409079868000);
    }
}
