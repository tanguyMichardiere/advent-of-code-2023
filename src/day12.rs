use itertools::Itertools;
use regex::bytes::Regex;

use crate::regex;

fn groups_regex(groups: &[usize]) -> Regex {
    Regex::new(&format!(
        r"^[.?]*{}[.?]*$",
        groups
            .iter()
            .map(|group| if group > &1 {
                format!(r"[#?]{{{}}}", group)
            } else {
                r"[#?]".to_owned()
            })
            .join(r"[.?]+")
    ))
    .unwrap()
}

fn parse(input: &str, fold: usize) -> impl Iterator<Item = (Vec<u8>, Regex)> + '_ {
    regex!(r"(?P<row>[.#\?]+) (?P<groups>[\d,]+)")
        .captures_iter(input)
        .map(move |caps| {
            let mut row = caps["row"].as_bytes().to_owned();
            let initial_row = row.clone();
            let mut groups = regex!(r"(?P<group>\d+)")
                .captures_iter(&caps["groups"])
                .map(|caps| caps["group"].parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            let initial_groups = groups.clone();
            for _ in 1..fold {
                row.push(b'?');
                row.extend(initial_row.iter());
                groups.extend(&initial_groups);
            }
            (row, groups_regex(&groups))
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

pub fn part_two(input: &str) -> String {
    // FIXME: complexity
    // parse(input, 5)
    //     .map(|(row, regex)| count_possibilities(row, &regex))
    //     .sum()
    format!(
        "{}\ncomplexity issue; this result is incorrect",
        parse(input, 1)
            .map(|(row, regex)| count_possibilities(row, &regex))
            .sum::<usize>()
    )
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
