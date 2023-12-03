use std::collections::{HashMap, HashSet};

use crate::regex;

pub fn part_one(input: &str) -> u32 {
    // HashMap<line_index, HashSet<indices_on_line>>
    let symbols = input
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            regex!(r"(?P<symbol>[^\d.])")
                .captures_iter(line)
                .map(move |caps| (line_index, caps.name("symbol").unwrap().start()))
        })
        .collect::<HashSet<_>>();
    input
        .lines()
        .enumerate()
        .map(|(line_index, line)| {
            regex!(r"(?P<number>\d+)")
                .captures_iter(line)
                .filter_map(|caps| {
                    let cap = caps.name("number").unwrap();
                    let index_range = cap.start().saturating_sub(1)..(cap.end() + 1);
                    if symbols.iter().any(|(symbol_line_index, symbol_index)| {
                        (line_index.saturating_sub(1)..(line_index + 2)).contains(symbol_line_index)
                            && index_range.contains(symbol_index)
                    }) {
                        Some(cap.as_str().parse::<u32>().unwrap())
                    } else {
                        None
                    }
                })
                .sum::<u32>()
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut stars = input
        .lines()
        .enumerate()
        .flat_map(|(line_index, line)| {
            regex!(r"(?P<star>\*)")
                .captures_iter(line)
                .map(move |caps| {
                    (
                        (line_index, caps.name("star").unwrap().start()),
                        Vec::<u32>::with_capacity(2),
                    )
                })
        })
        .collect::<HashMap<_, _>>();
    for (line_index, line) in input.lines().enumerate() {
        for caps in regex!(r"(?P<number>\d+)").captures_iter(line) {
            let cap = caps.name("number").unwrap();
            let index_range = cap.start().saturating_sub(1)..(cap.end() + 1);
            let number = cap.as_str().parse::<u32>().unwrap();
            for (_, gears) in stars
                .iter_mut()
                .filter(|((star_line_index, star_index), _)| {
                    (line_index.saturating_sub(1)..(line_index + 2)).contains(star_line_index)
                        && index_range.contains(star_index)
                })
            {
                gears.push(number);
            }
        }
    }
    stars
        .into_iter()
        .filter_map(|(_, gears)| {
            if gears.len() == 2 {
                Some(gears.into_iter().product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/03/1").unwrap();
        assert_eq!(part_one(&input), 4361);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/03/1").unwrap();
        assert_eq!(part_two(&input), 467835);
    }
}
