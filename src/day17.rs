use pathfinding::prelude::dijkstra;

use crate::grid::{Coordinates, Direction, Grid};

fn parse(input: &str) -> Grid<u32> {
    Grid::from_iter(
        input
            .lines()
            .map(|line| line.chars().map(|char| char.to_digit(10).unwrap())),
    )
}

pub fn part_one(input: &str) -> u32 {
    let map = parse(input);
    let start = Coordinates { x: 0, y: 0 };
    let finish = Coordinates {
        x: map.size.x - 1,
        y: map.size.y - 1,
    };
    dijkstra(
        &(start, Direction::Right, 0),
        |(position, current_direction, current_line_len)| {
            Direction::iter()
                .filter_map(|direction| {
                    if direction != &current_direction.reverse()
                        && (direction != current_direction || current_line_len < &3)
                    {
                        position.next(direction, &map.size).map(|position| {
                            let cost = map[&position];
                            (
                                (
                                    position,
                                    direction.clone(),
                                    if direction == current_direction {
                                        current_line_len + 1
                                    } else {
                                        1
                                    },
                                ),
                                cost,
                            )
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        },
        |(position, _, _)| position == &finish,
    )
    .unwrap()
    .1
}

pub fn part_two(input: &str) -> u32 {
    let map = parse(input);
    let start = Coordinates { x: 0, y: 0 };
    let finish = Coordinates {
        x: map.size.x - 1,
        y: map.size.y - 1,
    };
    dijkstra(
        &(start, Direction::Right, 0),
        |(position, current_direction, current_line_len)| {
            if current_line_len > &0 && current_line_len < &4 {
                position
                    .next(current_direction, &map.size)
                    .map(|position| {
                        let cost = map[&position];
                        vec![(
                            (position, current_direction.clone(), current_line_len + 1),
                            cost,
                        )]
                    })
                    .unwrap_or(Vec::new())
            } else {
                Direction::iter()
                    .filter_map(|direction| {
                        if current_line_len == &0
                            || (direction != &current_direction.reverse()
                                && (direction != current_direction || current_line_len < &10))
                        {
                            position.next(direction, &map.size).map(|position| {
                                let cost = map[&position];
                                (
                                    (
                                        position,
                                        direction.clone(),
                                        if direction == current_direction {
                                            current_line_len + 1
                                        } else {
                                            1
                                        },
                                    ),
                                    cost,
                                )
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            }
        },
        |(position, _, current_line_len)| position == &finish && current_line_len >= &4,
    )
    .unwrap()
    .1
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/17/1").unwrap();
        assert_eq!(part_one(&input), 102);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/17/1").unwrap();
        assert_eq!(part_two(&input), 94);
        let input = read_to_string("examples/17/2").unwrap();
        assert_eq!(part_two(&input), 71);
    }
}
