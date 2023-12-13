use cached::proc_macro::cached;

use crate::regex;

fn parse(input: &str, fold: usize) -> impl Iterator<Item = (Vec<u8>, Vec<usize>)> + '_ {
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
            (row, groups)
        })
}

// credits: https://github.com/maksverver/AdventOfCode/blob/master/2023/12.py
#[cached(key = "u64", convert = "{ crate::cache::hash((row, groups)) }")]
fn count_possibilities(row: &[u8], groups: &[usize]) -> usize {
    if groups.is_empty() {
        return if row.contains(&b'#') { 0 } else { 1 };
    }
    let group = groups[0];
    if row.len() < group {
        return 0;
    }
    let mut result = 0;
    if row[0] != b'#' {
        result += count_possibilities(&row[1..], groups);
    }
    if !row[..group].contains(&b'.') && row[group] != b'#' {
        result += count_possibilities(&row[(group + 1)..], &groups[1..]);
    }
    result
}

pub fn part_one(input: &str) -> usize {
    parse(input, 1)
        .map(|(mut row, groups)| {
            row.push(b'.');
            count_possibilities(&row, &groups)
        })
        .sum()
}

pub fn part_two(input: &str) -> usize {
    parse(input, 5)
        .map(|(mut row, groups)| {
            row.push(b'.');
            count_possibilities(&row, &groups)
        })
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
