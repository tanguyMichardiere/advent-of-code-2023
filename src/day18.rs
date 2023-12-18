use itertools::Itertools;

use crate::grid::Direction;
use crate::regex;

type Instruction = (Direction, usize);

fn points(instructions: &[Instruction]) -> Vec<(i64, i64)> {
    let mut points = vec![(0, 0)];
    for (direction, distance) in instructions {
        let previous_point = points.last().unwrap();
        points.push(match direction {
            Direction::Up => (previous_point.0, previous_point.1 - *distance as i64),
            Direction::Right => (previous_point.0 + *distance as i64, previous_point.1),
            Direction::Down => (previous_point.0, previous_point.1 + *distance as i64),
            Direction::Left => (previous_point.0 - *distance as i64, previous_point.1),
        })
    }
    points
}

fn inner_area(points: &[(i64, i64)]) -> u64 {
    points
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum::<i64>()
        .abs() as u64
        / 2
}

// the shoelace algorithm doesn't take the borders into account
fn outer_area(instructions: &[Instruction]) -> u64 {
    let number_of_borders = instructions.len() as u64;
    let inward_corners = (number_of_borders - 4) / 2;
    let outward_corners = inward_corners + 4;
    let total_border_len = instructions
        .iter()
        .map(|(_, distance)| distance)
        .sum::<usize>() as u64;
    (2 * (total_border_len - number_of_borders) + 3 * outward_corners + inward_corners) / 4
}

pub fn part_one(input: &str) -> u64 {
    let instructions = regex!(r"(?P<direction>[URDL]) (?P<distance>\d+) \(#[[:xdigit:]]{6}\)")
        .captures_iter(input)
        .map(|caps| {
            (
                match &caps["direction"] {
                    "U" => Direction::Up,
                    "R" => Direction::Right,
                    "D" => Direction::Down,
                    "L" => Direction::Left,
                    _ => unreachable!(),
                },
                caps["distance"].parse().unwrap(),
            )
        })
        .collect::<Vec<Instruction>>();
    inner_area(&points(&instructions)) + outer_area(&instructions)
}

pub fn part_two(input: &str) -> u64 {
    let instructions =
        regex!(r"[URDL] \d+ \(#(?P<distance>[[:xdigit:]]{5})(?P<direction>[[:xdigit:]])\)")
            .captures_iter(input)
            .map(|caps| {
                (
                    match &caps["direction"] {
                        "3" => Direction::Up,
                        "0" => Direction::Right,
                        "1" => Direction::Down,
                        "2" => Direction::Left,
                        _ => unreachable!(),
                    },
                    usize::from_str_radix(&caps["distance"], 16).unwrap(),
                )
            })
            .collect::<Vec<Instruction>>();
    inner_area(&points(&instructions)) + outer_area(&instructions)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/18/1").unwrap();
        assert_eq!(part_one(&input), 62);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/18/1").unwrap();
        assert_eq!(part_two(&input), 952408144115);
    }
}
