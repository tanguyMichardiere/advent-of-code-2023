use std::collections::HashSet;

use itertools::Itertools;

use crate::regex;

fn parse(input: &str) -> impl Iterator<Item = Vec<Vec<u8>>> + '_ {
    regex!(r"(?P<pattern>(?:[.#]+\n)+)")
        .captures_iter(input)
        .map(|caps| {
            regex!(r"(?P<line>[.#]+)")
                .captures_iter(&caps["pattern"])
                .map(|caps| caps["line"].as_bytes().to_vec())
                .collect::<Vec<_>>()
        })
}

fn is_valid_reflection(line: &[u8], position: usize) -> bool {
    line[..(position + 1)]
        .iter()
        .rev()
        .zip(line[(position + 1)..].iter())
        .all(|(a, b)| a == b)
}

fn refine_reflections(line: &[u8], positions: &mut Vec<usize>) {
    positions.retain(|position| is_valid_reflection(line, *position));
}

fn find_horizontal_reflection(pattern: &[Vec<u8>]) -> Option<usize> {
    let mut reflections = (0..(pattern[0].len() - 1)).collect::<Vec<_>>();
    for line in pattern {
        refine_reflections(line, &mut reflections);
        if reflections.is_empty() {
            return None;
        }
    }
    Some(reflections.into_iter().exactly_one().unwrap())
}

fn rotated(pattern: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut rotated_pattern = vec![Vec::new(); pattern[0].len()];
    for line in pattern {
        for x in 0..line.len() {
            rotated_pattern[x].push(line[x]);
        }
    }
    rotated_pattern
}

fn find_vertical_reflection(pattern: &[Vec<u8>]) -> Option<usize> {
    find_horizontal_reflection(&rotated(pattern))
}

fn find_reflection(pattern: &[Vec<u8>]) -> (usize, bool) {
    if let Some(position) = find_horizontal_reflection(pattern) {
        (position, false)
    } else {
        (find_vertical_reflection(pattern).unwrap(), true)
    }
}

pub fn part_one(input: &str) -> usize {
    parse(input)
        .map(|pattern| {
            let (position, vertical) = find_reflection(&pattern);
            (position + 1) * if vertical { 100 } else { 1 }
        })
        .sum()
}

fn refine_reflections_with_smudge(line: &[u8], positions: &mut Vec<usize>) {
    let mut reflections = HashSet::new();
    for smudge in 0..line.len() {
        let mut line = line.to_vec();
        line[smudge] = if line[smudge] == b'.' { b'#' } else { b'.' };
        for position in 0..(line.len() - 1) {
            if is_valid_reflection(&line, position) {
                reflections.insert(position);
            }
        }
    }
    positions.retain(|position| reflections.contains(position));
}

fn find_horizontal_reflections_with_smudge(pattern: &[Vec<u8>]) -> Vec<usize> {
    let mut reflections = Vec::new();
    for smudge in 0..pattern.len() {
        let mut smudge_reflections = (0..(pattern[0].len() - 1)).collect::<Vec<_>>();
        for (i, line) in pattern.iter().enumerate() {
            if i == smudge {
                refine_reflections_with_smudge(line, &mut smudge_reflections);
            } else {
                refine_reflections(line, &mut smudge_reflections);
            }
        }
        reflections.extend(smudge_reflections.into_iter());
    }
    reflections
}

fn find_vertical_reflections_with_smudge(pattern: &[Vec<u8>]) -> Vec<usize> {
    find_horizontal_reflections_with_smudge(&rotated(pattern))
}

fn find_reflection_with_smudge(pattern: &[Vec<u8>]) -> (usize, bool) {
    let reflection_without_smudge = find_reflection(pattern);
    if let Ok(position) = find_horizontal_reflections_with_smudge(pattern)
        .into_iter()
        .filter(|position| reflection_without_smudge.1 || position != &reflection_without_smudge.0)
        .exactly_one()
    {
        (position, false)
    } else {
        (
            find_vertical_reflections_with_smudge(pattern)
                .into_iter()
                .filter(|position| {
                    !reflection_without_smudge.1 || position != &reflection_without_smudge.0
                })
                .exactly_one()
                .unwrap(),
            true,
        )
    }
}

pub fn part_two(input: &str) -> usize {
    parse(input)
        .map(|pattern| {
            let (position, vertical) = find_reflection_with_smudge(&pattern);
            (position + 1) * if vertical { 100 } else { 1 }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/13/1").unwrap();
        assert_eq!(part_one(&input), 405);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/13/1").unwrap();
        assert_eq!(part_two(&input), 400);
    }
}
