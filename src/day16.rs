#[derive(Clone)]
enum Mirror {
    Right, // /
    Left,  // \
}

#[derive(Clone)]
enum Splitter {
    Vertical,   // |
    Horizontal, // -
}

#[derive(Clone)]
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

type Grid = Vec<Vec<(Option<Object>, Vec<Direction>)>>;

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| {
                    (
                        match char {
                            '.' => None,
                            '/' => Some(Object::Mirror(Mirror::Right)),
                            '\\' => Some(Object::Mirror(Mirror::Left)),
                            '|' => Some(Object::Splitter(Splitter::Vertical)),
                            '-' => Some(Object::Splitter(Splitter::Horizontal)),
                            _ => unreachable!(),
                        },
                        Vec::with_capacity(4),
                    )
                })
                .collect()
        })
        .collect()
}

fn advance_beam(grid: &mut Grid, (x, y): Coordinates, direction: Direction) {
    let size = grid.len();
    if !grid[y][x].1.contains(&direction) {
        grid[y][x].1.push(direction.clone());
        match &grid[y][x].0 {
            None => {
                if let Some((x, y)) = direction.next((x, y), size) {
                    advance_beam(grid, (x, y), direction);
                }
            }
            Some(Object::Mirror(mirror)) => {
                let direction = direction.reflect(mirror);
                if let Some((x, y)) = direction.next((x, y), size) {
                    advance_beam(grid, (x, y), direction);
                }
            }
            Some(Object::Splitter(splitter)) => {
                for direction in direction.split(splitter) {
                    if let Some((x, y)) = direction.next((x, y), size) {
                        advance_beam(grid, (x, y), direction);
                    }
                }
            }
        }
    }
}

fn energy(grid: &Grid) -> usize {
    grid.iter()
        .map(|row| row.iter().filter(|(_, beams)| !beams.is_empty()).count())
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let mut grid = parse(input);
    advance_beam(&mut grid, (0, 0), Direction::Right);
    energy(&grid)
}

pub fn part_two(input: &str) -> usize {
    let grid = parse(input);
    let size = grid.len();
    let mut max_energy = 0;
    for i in 0..size {
        {
            let mut grid = grid.clone();
            advance_beam(&mut grid, (0, i), Direction::Right);
            max_energy = max_energy.max(energy(&grid));
        }
        {
            let mut grid = grid.clone();
            advance_beam(&mut grid, (i, size - 1), Direction::Up);
            max_energy = max_energy.max(energy(&grid));
        }
        {
            let mut grid = grid.clone();
            advance_beam(&mut grid, (size - 1, i), Direction::Left);
            max_energy = max_energy.max(energy(&grid));
        }
        {
            let mut grid = grid.clone();
            advance_beam(&mut grid, (i, 0), Direction::Down);
            max_energy = max_energy.max(energy(&grid));
        }
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
