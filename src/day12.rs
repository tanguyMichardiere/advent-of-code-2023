use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;

use crate::regex;

fn parse(input: &str) -> impl Iterator<Item = (Vec<Option<bool>>, Vec<usize>)> + '_ {
    regex!(r"(?P<row>[.#\?]+) (?P<groups>[\d,]+)")
        .captures_iter(input)
        .map(|caps| {
            (
                caps["row"]
                    .chars()
                    .map(|char| match char {
                        '.' => Some(false),
                        '#' => Some(true),
                        '?' => None,
                        _ => unreachable!(),
                    })
                    .collect(),
                regex!(r"(?P<group>\d+)")
                    .captures_iter(&caps["groups"])
                    .map(|caps| caps["group"].parse::<usize>().unwrap())
                    .collect(),
            )
        })
}

fn count(row: &[Option<bool>]) -> (usize, usize, usize) {
    let mut operational_count = 0;
    let mut damaged_count = 0;
    let mut unknown_count = 0;
    for damaged in row {
        match damaged {
            Some(false) => operational_count += 1,
            Some(true) => damaged_count += 1,
            None => unknown_count += 1,
        }
    }
    (operational_count, damaged_count, unknown_count)
}

fn is_valid(row: &[Option<bool>], expected_groups: &[usize]) -> bool {
    let expected_damaged_count = expected_groups.iter().sum::<usize>();
    let (_, damaged_count, unknown_count) = count(row);
    if damaged_count == expected_damaged_count {
        let mut groups = vec![0];
        for damaged in row {
            if damaged.is_some_and(|damaged| damaged) {
                *groups.last_mut().unwrap() += 1;
            } else {
                groups.push(0);
            }
        }
        groups.retain(|group| group != &0);
        return groups == expected_groups;
    }
    if damaged_count > expected_damaged_count {
        return false;
    }
    if damaged_count + unknown_count < expected_damaged_count {
        return false;
    }
    if (damaged_count + unknown_count) == expected_damaged_count {
        return is_valid(
            &row.into_iter()
                .map(|damaged| damaged.or(Some(true)))
                .collect::<Vec<_>>(),
            expected_groups,
        );
    }
    let mut groups = vec![0];
    for damaged in row {
        match damaged {
            Some(false) => groups.push(0),
            Some(true) => *groups.last_mut().unwrap() += 1,
            None => {
                break;
            }
        }
    }
    let last_group = groups.remove(groups.len() - 1);
    groups.retain(|group| group > &0);
    if groups.len() + 1 > expected_groups.len() {
        return false;
    }
    if last_group > expected_groups[groups.len()] {
        return false;
    }
    if groups != expected_groups[..groups.len()] {
        return false;
    }
    for group_size in 1..row.len() {
        let number_of_groups_this_size = [&[Some(false)], row, &[Some(false)]]
            .concat()
            .windows(group_size + 2)
            .filter(|window| {
                window.starts_with(&[Some(false)])
                    && window[1..window.len() - 1]
                        .into_iter()
                        .all_equal_value()
                        .is_ok_and(|value| value == &Some(true))
                    && window.ends_with(&[Some(false)])
            })
            .count();
        if number_of_groups_this_size
            > expected_groups
                .iter()
                .filter(|group| group == &&group_size)
                .count()
        {
            return false;
        }
    }
    true
}

fn count_possibilities(row: Vec<Option<bool>>, expected_groups: &[usize]) -> usize {
    match row.iter().find_position(|damaged| damaged.is_none()) {
        Some((first_unknown_position, _)) => {
            let mut with_false = row.clone();
            with_false[first_unknown_position] = Some(false);
            let mut with_true = row;
            with_true[first_unknown_position] = Some(true);
            let mut total = 0;
            if is_valid(&with_false, &expected_groups) {
                total += count_possibilities(with_false, expected_groups);
            }
            if is_valid(&with_true, &expected_groups) {
                total += count_possibilities(with_true, expected_groups);
            }
            total
        }
        None => 1,
    }
}

pub fn part_one(input: &str) -> usize {
    parse(input)
        .map(|(row, groups)| count_possibilities(row, &groups))
        .sum()
}

const FOLD: usize = 5;

fn unfold((mut row, groups): (Vec<Option<bool>>, Vec<usize>)) -> (Vec<Option<bool>>, Vec<usize>) {
    let initial_row = row.clone();
    for _ in 0..(FOLD - 1) {
        row.push(None);
        row.extend(initial_row.iter());
    }
    (row, groups.repeat(FOLD))
}

pub fn part_two(input: &str) -> usize {
    parse(input)
        .par_bridge()
        .map(unfold)
        .map(|(row, groups)| count_possibilities(row, &groups))
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

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/12/1").unwrap();
        assert_eq!(part_two(&input), 525152);
    }
}
