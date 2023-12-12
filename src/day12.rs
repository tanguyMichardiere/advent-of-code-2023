use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use regex::bytes::Regex;

use crate::regex;

fn parse(input: &str, fold: usize) -> impl Iterator<Item = (Vec<u8>, Regex)> + '_ {
    regex!(r"(?P<row>[.#\?]+) (?P<groups>[\d,]+)")
        .captures_iter(input)
        .map(move |caps| {
            let mut row = caps["row"].as_bytes().to_owned();
            let initial_row = row.clone();
            let mut inner_pattern = regex!(r"(?P<group>\d+)")
                .captures_iter(&caps["groups"])
                .map(|caps| format!(r"[#?]{{{}}}", caps["group"].parse::<usize>().unwrap()))
                .join(r"[.?]+");
            let initial_inner_pattern = inner_pattern.clone();
            for _ in 1..fold {
                row.push(b'?');
                row.extend(initial_row.iter());
                inner_pattern.push_str(r"[.?]+");
                inner_pattern.push_str(&initial_inner_pattern);
            }
            (
                row,
                Regex::new(&format!(r"^[.?]*{}[.?]*$", inner_pattern)).unwrap(),
            )
        })
}

fn count_possibilities(mut row: Vec<u8>, regex: &Regex) -> usize {
    match row.iter().find_position(|char| char == &&b'?') {
        Some((first_unknown_position, _)) => {
            row[first_unknown_position] = b'.';
            let with_dot_is_valid = regex.is_match(&row);
            row[first_unknown_position] = b'#';
            let with_hash_is_valid = regex.is_match(&row);
            match (with_dot_is_valid, with_hash_is_valid) {
                (false, false) => 0,
                (false, true) => count_possibilities(row, regex),
                (true, false) => {
                    row[first_unknown_position] = b'.';
                    count_possibilities(row, regex)
                }
                (true, true) => {
                    // only clone `row` if it's really necessary
                    let mut with_dot = row.clone();
                    with_dot[first_unknown_position] = b'.';
                    count_possibilities(with_dot, regex) + count_possibilities(row, regex)
                }
            }
        }
        None => 1,
    }
}

pub fn part_one(input: &str) -> usize {
    parse(input, 1)
        .map(|(row, regex)| count_possibilities(row, &regex))
        .sum()
}

pub fn part_two(input: &str) -> usize {
    // FIXME: complexity
    parse(input, 1)
        .par_bridge()
        .map(|(row, regex)| count_possibilities(row, &regex))
        // .progress_count(input.lines().count() as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/12/1").unwrap();
        assert_eq!(part_one(&input), 21);
    }

    // #[test]
    // fn test_part_two() {
    //     let input = read_to_string("examples/12/1").unwrap();
    //     assert_eq!(part_two(&input), 525152);
    // }
}
