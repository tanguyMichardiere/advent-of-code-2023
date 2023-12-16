enum Mirror {
    Right, // /
    Left,  // \
}

enum Splitter {
    Vertical,   // |
    Horizontal, // -
}

enum Object {
    Mirror(Mirror),
    Splitter(Splitter),
}

type Coordinates = (usize, usize);

#[derive(Clone, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn reflect(&self, mirror: &Mirror) -> Self {
        match (self, mirror) {
            (Self::Up, Mirror::Right) | (Self::Down, Mirror::Left) => Self::Right,
            (Self::Up, Mirror::Left) | (Self::Down, Mirror::Right) => Self::Left,
            (Self::Right, Mirror::Right) | (Self::Left, Mirror::Left) => Self::Up,
            (Self::Right, Mirror::Left) | (Self::Left, Mirror::Right) => Self::Down,
        }
    }

    fn split(&self, splitter: &Splitter) -> Vec<Self> {
        match (self, splitter) {
            (Self::Up, Splitter::Vertical)
            | (Self::Right, Splitter::Horizontal)
            | (Self::Down, Splitter::Vertical)
            | (Self::Left, Splitter::Horizontal) => vec![self.clone()],
            (Self::Up, Splitter::Horizontal) | (Self::Down, Splitter::Horizontal) => {
                vec![Self::Left, Self::Right]
            }
            (Self::Right, Splitter::Vertical) | (Self::Left, Splitter::Vertical) => {
                vec![Self::Up, Self::Down]
            }
        }
    }

    fn next(&self, (x, y): Coordinates, size: usize) -> Option<Coordinates> {
        match self {
            Self::Up => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            }
            Self::Right => {
                if x < size - 1 {
                    Some((x + 1, y))
                } else {
                    None
                }
            }
            Self::Down => {
                if y < size - 1 {
                    Some((x, y + 1))
                } else {
                    None
                }
            }
            Self::Left => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            }
        }
    }
}

struct Tile {
    object: Option<Object>,
    beams: Vec<Direction>,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self {
            object: match value {
                '.' => None,
                '/' => Some(Object::Mirror(Mirror::Right)),
                '\\' => Some(Object::Mirror(Mirror::Left)),
                '|' => Some(Object::Splitter(Splitter::Vertical)),
                '-' => Some(Object::Splitter(Splitter::Horizontal)),
                _ => unreachable!(),
            },
            beams: Vec::with_capacity(4),
        }
    }
}

struct Grid {
    tiles: Vec<Vec<Tile>>,
    size: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let tiles = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let size = tiles.len();
        Self { tiles, size }
    }

    fn advance_beam(&mut self, coordinates: Coordinates, direction: Direction) {
        let Tile { object, beams } = &mut self.tiles[coordinates.1][coordinates.0];
        if !beams.contains(&direction) {
            beams.push(direction.clone());
            match &object {
                None => {
                    if let Some(coordinates) = direction.next(coordinates, self.size) {
                        self.advance_beam(coordinates, direction);
                    }
                }
                Some(Object::Mirror(mirror)) => {
                    let direction = direction.reflect(mirror);
                    if let Some(coordinates) = direction.next(coordinates, self.size) {
                        self.advance_beam(coordinates, direction);
                    }
                }
                Some(Object::Splitter(splitter)) => {
                    for direction in direction.split(splitter) {
                        if let Some(coordinates) = direction.next(coordinates, self.size) {
                            self.advance_beam(coordinates, direction);
                        }
                    }
                }
            }
        }
    }

    fn energy(&self) -> usize {
        self.tiles
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|Tile { beams, .. }| !beams.is_empty())
                    .count()
            })
            .sum()
    }

    fn reset(&mut self) {
        for row in &mut self.tiles {
            for tile in row {
                tile.beams.truncate(0);
            }
        }
    }
}

pub fn part_one(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    grid.advance_beam((0, 0), Direction::Right);
    grid.energy()
}

pub fn part_two(input: &str) -> usize {
    let mut grid = Grid::parse(input);
    let mut max_energy = 0;
    for i in 0..grid.size {
        grid.advance_beam((0, i), Direction::Right);
        max_energy = max_energy.max(grid.energy());
        grid.reset();
        grid.advance_beam((i, grid.size - 1), Direction::Up);
        max_energy = max_energy.max(grid.energy());
        grid.reset();
        grid.advance_beam((grid.size - 1, i), Direction::Left);
        max_energy = max_energy.max(grid.energy());
        grid.reset();
        grid.advance_beam((i, 0), Direction::Down);
        max_energy = max_energy.max(grid.energy());
        grid.reset();
    }
    max_energy
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/16/1").unwrap();
        assert_eq!(part_one(&input), 46);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/16/1").unwrap();
        assert_eq!(part_two(&input), 51);
    }
}
