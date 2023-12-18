use std::{iter, ops, slice};

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

pub struct Grid<C> {
    cells: Vec<Vec<C>>,
    pub size: Coordinates,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Coordinates {
    pub fn next(&self, direction: &Direction, size: &Coordinates) -> Option<Self> {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    Some(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < size.x - 1 {
                    Some(Self {
                        x: self.x + 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < size.y - 1 {
                    Some(Self {
                        x: self.x,
                        y: self.y + 1,
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some(Self {
                        x: self.x - 1,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }

    pub fn neighbors<'a>(
        &'a self,
        size: &'a Coordinates,
    ) -> impl Iterator<Item = (Self, Direction)> + 'a {
        Direction::iter().filter_map(|direction| {
            self.next(direction, size)
                .map(|position| (position, direction.clone()))
        })
    }

    pub fn all_neighbors<'a>(
        positions: impl IntoIterator<Item = &'a Self> + 'a,
        size: &'a Coordinates,
    ) -> impl Iterator<Item = (Self, Direction)> + 'a {
        positions
            .into_iter()
            .flat_map(|position| position.neighbors(size))
    }
}

impl<C, A: IntoIterator<Item = C>> FromIterator<A> for Grid<C> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let cells: Vec<Vec<C>> = iter.into_iter().map(|i| i.into_iter().collect()).collect();
        debug_assert!(cells.iter().map(|row| row.len()).all_equal());
        let size = Coordinates {
            x: cells.first().map_or(0, |row| row.len()),
            y: cells.len(),
        };
        Self { cells, size }
    }
}

impl<C> ops::Index<&Coordinates> for Grid<C> {
    type Output = C;

    fn index(&self, index: &Coordinates) -> &Self::Output {
        self.cells.index(index.y).index(index.x)
    }
}

impl<C> ops::IndexMut<&Coordinates> for Grid<C> {
    fn index_mut(&mut self, index: &Coordinates) -> &mut Self::Output {
        self.cells.index_mut(index.y).index_mut(index.x)
    }
}

impl<'a, C> IntoIterator for &'a Grid<C> {
    type Item = (Coordinates, &'a C);

    type IntoIter = Box<dyn Iterator<Item = Self::Item> + 'a>;

    fn into_iter(self) -> Self::IntoIter {
        Box::new(self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, cell)| (Coordinates { x, y }, cell))
        }))
    }
}

impl<C> Grid<C> {
    pub fn clone_with<T: Clone>(&self, value: T) -> Grid<T> {
        Grid::from_iter(iter::repeat(iter::repeat(value).take(self.size.x)).take(self.size.y))
    }
}

impl Direction {
    pub fn iter() -> slice::Iter<'static, Self> {
        static DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];
        DIRECTIONS.iter()
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
        }
    }
}
