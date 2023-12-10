use std::cmp::Ordering::{Greater, Less};
use std::ops::{Index, IndexMut};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq)]
enum Pipe {
    /// |
    NS,
    /// -
    EW,
    /// L
    NE,
    /// J
    NW,
    /// 7
    SW,
    /// F
    SE,
}

impl Pipe {
    fn is_connected_to_north(&self) -> bool {
        self == &Self::NS || self == &Self::NE || self == &Self::NW
    }

    fn is_connected_to_west(&self) -> bool {
        self == &Self::EW || self == &Self::NW || self == &Self::SW
    }

    fn is_connected_to_south(&self) -> bool {
        self == &Self::NS || self == &Self::SW || self == &Self::SE
    }

    fn is_connected_to_east(&self) -> bool {
        self == &Self::EW || self == &Self::NE || self == &Self::SE
    }

    fn separates_north_west_and_south_east(&self) -> bool {
        self == &Self::NS || self == &Self::EW || self == &Self::NW || self == &Self::SE
    }
}

#[derive(PartialEq)]
enum Tile {
    Pipe(Pipe),
    Ground,
    Start,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Pipe(Pipe::NS),
            '-' => Self::Pipe(Pipe::EW),
            'L' => Self::Pipe(Pipe::NE),
            'J' => Self::Pipe(Pipe::NW),
            '7' => Self::Pipe(Pipe::SW),
            'F' => Self::Pipe(Pipe::SE),
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy)]
struct Coordinates(usize, usize);

impl Coordinates {
    fn north(&self) -> Self {
        Self(self.0, self.1 - 1)
    }

    fn west(&self) -> Self {
        Self(self.0 - 1, self.1)
    }

    fn south(&self) -> Self {
        Self(self.0, self.1 + 1)
    }

    fn east(&self) -> Self {
        Self(self.0 + 1, self.1)
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    start: Coordinates,
    size: Coordinates,
}

impl Index<Coordinates> for Grid {
    type Output = Tile;

    fn index(&self, index: Coordinates) -> &Self::Output {
        self.tiles.index(index.1).index(index.0)
    }
}

impl IndexMut<Coordinates> for Grid {
    fn index_mut(&mut self, index: Coordinates) -> &mut Self::Output {
        self.tiles.index_mut(index.1).index_mut(index.0)
    }
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert!(tiles.iter().map(|row| row.len()).all_equal());
        let size = Coordinates(tiles[0].len(), tiles.len());
        for (y, row) in tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile == &Tile::Start {
                    return Self {
                        tiles,
                        start: Coordinates(x, y),
                        size,
                    };
                }
            }
        }
        unreachable!()
    }

    fn costs(&self) -> Vec<Vec<usize>> {
        fn recursion(
            grid: &Grid,
            costs: &mut Vec<Vec<usize>>,
            current: Coordinates,
            previous: Coordinates,
        ) {
            if let Tile::Pipe(pipe) = grid[current] {
                costs[current.1][current.0] =
                    costs[current.1][current.0].min(costs[previous.1][previous.0] + 1);
                match (pipe, current.0.cmp(&previous.0), current.1.cmp(&previous.1)) {
                    (Pipe::NS, _, Greater) | (Pipe::SW, Greater, _) | (Pipe::SE, Less, _) => {
                        recursion(grid, costs, current.south(), current)
                    }
                    (Pipe::EW, Greater, _) | (Pipe::NE, _, Greater) | (Pipe::SE, _, Less) => {
                        recursion(grid, costs, current.east(), current)
                    }
                    (Pipe::NS, _, Less) | (Pipe::NE, Less, _) | (Pipe::NW, Greater, _) => {
                        recursion(grid, costs, current.north(), current)
                    }
                    (Pipe::EW, Less, _) | (Pipe::NW, _, Greater) | (Pipe::SW, _, Less) => {
                        recursion(grid, costs, current.west(), current)
                    }
                    _ => unreachable!(),
                }
            }
        }

        let mut costs = vec![vec![usize::MAX; self.size.0]; self.size.1];
        costs[self.start.1][self.start.0] = 0;
        if self.start.1 > 0 {
            let south = self.start.north();
            if let Tile::Pipe(pipe) = self[south] {
                if pipe.is_connected_to_south() {
                    recursion(self, &mut costs, south, self.start);
                }
            }
        }
        if self.start.0 > 0 {
            let west = self.start.west();
            if let Tile::Pipe(pipe) = self[west] {
                if pipe.is_connected_to_east() {
                    recursion(self, &mut costs, west, self.start);
                }
            }
        }
        if self.start.1 < self.size.1 {
            let south = self.start.south();
            if let Tile::Pipe(pipe) = self[south] {
                if pipe.is_connected_to_north() {
                    recursion(self, &mut costs, south, self.start);
                }
            }
        }
        if self.start.0 < self.size.0 {
            let east = self.start.east();
            if let Tile::Pipe(pipe) = self[east] {
                if pipe.is_connected_to_west() {
                    recursion(self, &mut costs, east, self.start);
                }
            }
        }
        costs
    }

    fn stripped_pipes(self) -> Vec<Vec<Option<Pipe>>> {
        let costs = self.costs();

        let is_connected_to_north = {
            if self.start.1 > 0 {
                if let Tile::Pipe(pipe) = self[self.start.north()] {
                    pipe.is_connected_to_south()
                } else {
                    false
                }
            } else {
                false
            }
        };
        let is_connected_to_west = {
            if self.start.0 > 0 {
                if let Tile::Pipe(pipe) = self[self.start.west()] {
                    pipe.is_connected_to_east()
                } else {
                    false
                }
            } else {
                false
            }
        };
        let is_connected_to_south = {
            if self.start.1 < self.size.1 {
                if let Tile::Pipe(pipe) = self[self.start.south()] {
                    pipe.is_connected_to_north()
                } else {
                    false
                }
            } else {
                false
            }
        };
        let is_connected_to_east = {
            if self.start.0 < self.size.0 {
                if let Tile::Pipe(pipe) = self[self.start.east()] {
                    pipe.is_connected_to_west()
                } else {
                    false
                }
            } else {
                false
            }
        };
        let start = match (
            is_connected_to_north,
            is_connected_to_west,
            is_connected_to_south,
            is_connected_to_east,
        ) {
            (true, false, true, false) => Pipe::NS,
            (false, true, false, true) => Pipe::EW,
            (true, false, false, true) => Pipe::NE,
            (true, true, false, false) => Pipe::NW,
            (false, true, true, false) => Pipe::SW,
            (false, false, true, true) => Pipe::SE,
            _ => unreachable!(),
        };

        self.tiles
            .into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .zip(&costs[y])
                    .map(|(tile, cost)| {
                        if cost < &usize::MAX {
                            if let Tile::Pipe(pipe) = tile {
                                Some(pipe)
                            } else if tile == Tile::Start {
                                Some(start)
                            } else {
                                unreachable!()
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect()
    }
}

pub fn part_one(input: &str) -> usize {
    Grid::parse(input)
        .costs()
        .into_iter()
        .flatten()
        .filter(|cost| cost < &usize::MAX)
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> usize {
    let mut pipes = Grid::parse(input)
        .stripped_pipes()
        .into_iter()
        .map(|row| row.iter().map(|tile| (*tile, false)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for y in 1..pipes.len() {
        for x in 1..pipes[y].len() {
            let north_west_inside = pipes[y - 1][x - 1].1;
            if let Some(pipe) = pipes[y - 1][x - 1].0 {
                if pipe.separates_north_west_and_south_east() {
                    pipes[y][x].1 = !north_west_inside;
                } else {
                    pipes[y][x].1 = north_west_inside;
                }
            } else {
                pipes[y][x].1 = north_west_inside;
            }
        }
    }
    pipes
        .into_iter()
        .map(|row| {
            row.into_iter()
                .filter(|(tile, inside)| *inside && tile.is_none())
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/10/1").unwrap();
        assert_eq!(part_one(&input), 4);

        let input = read_to_string("examples/10/2").unwrap();
        assert_eq!(part_one(&input), 8);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/10/3").unwrap();
        assert_eq!(part_two(&input), 4);

        let input = read_to_string("examples/10/4").unwrap();
        assert_eq!(part_two(&input), 8);

        let input = read_to_string("examples/10/5").unwrap();
        assert_eq!(part_two(&input), 10);
    }
}
