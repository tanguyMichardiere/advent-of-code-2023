use itertools::Itertools;

use crate::regex;

fn parse_and_extrapolate(input: &str) -> impl Iterator<Item = Vec<Vec<i32>>> + '_ {
    regex!(r"(?P<sequence>(?:[-\d ])+)")
        .captures_iter(input)
        .map(|caps| {
            let mut sequences = vec![regex!(r"(?P<number>-?\d+)")
                .captures_iter(&caps["sequence"])
                .map(|caps| caps["number"].parse::<i32>().unwrap())
                .collect::<Vec<_>>()];
            while sequences.last().unwrap().iter().all_equal_value() != Ok(&0) {
                sequences.push(
                    sequences
                        .last()
                        .unwrap()
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| b - a)
                        .collect(),
                );
            }
            sequences
        })
}

pub fn part_one(input: &str) -> i32 {
    parse_and_extrapolate(input)
        .map(|sequences| {
            sequences
                .into_iter()
                .map(|sequence| *sequence.last().unwrap())
                .sum::<i32>()
        })
        .sum()
}

pub fn part_two(input: &str) -> i32 {
    parse_and_extrapolate(input)
        .map(|sequences| {
            sequences
                .into_iter()
                .map(|sequence| *sequence.first().unwrap())
                .rev()
                .reduce(|a, b| b - a)
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/09/1").unwrap();
        assert_eq!(part_one(&input), 114);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/09/1").unwrap();
        assert_eq!(part_two(&input), 2);
    }
}
