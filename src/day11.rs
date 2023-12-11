use std::collections::HashSet;

use itertools::Itertools;

fn parse_galaxies(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, char)| if char == '#' { Some((x, y)) } else { None })
        })
        .collect()
}

fn shift(galaxies: &mut Vec<(usize, usize)>, expansion: usize) {
    let non_empty_columns = galaxies.iter().map(|(x, _)| *x).collect::<HashSet<_>>();
    let non_empty_rows = galaxies.iter().map(|(_, y)| *y).collect::<HashSet<_>>();
    let mut column_shift = 0;
    for empty_column in
        (1..*non_empty_columns.iter().max().unwrap()).filter(|x| !non_empty_columns.contains(x))
    {
        for galaxy in galaxies.iter_mut() {
            if galaxy.0 > empty_column + column_shift {
                galaxy.0 += expansion;
            }
        }
        column_shift += expansion;
    }
    let mut row_shift = 0;
    for empty_row in
        (1..*non_empty_rows.iter().max().unwrap()).filter(|y| !non_empty_rows.contains(y))
    {
        for galaxy in galaxies.iter_mut() {
            if galaxy.1 > empty_row + row_shift {
                galaxy.1 += expansion;
            }
        }
        row_shift += expansion;
    }
}

fn distances_sum(galaxies: &Vec<(usize, usize)>) -> usize {
    galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.0.abs_diff(b.0) + a.1.abs_diff(b.1))
        .sum()
}

pub fn part_one(input: &str) -> usize {
    let mut galaxies = parse_galaxies(input);
    shift(&mut galaxies, 1);
    distances_sum(&galaxies)
}

pub fn part_two(input: &str) -> usize {
    let mut galaxies = parse_galaxies(input);
    shift(&mut galaxies, 999999);
    distances_sum(&galaxies)
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/11/1").unwrap();
        assert_eq!(part_one(&input), 374);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/11/1").unwrap();

        let mut galaxies = parse_galaxies(&input);
        shift(&mut galaxies, 9);
        assert_eq!(distances_sum(&galaxies), 1030);

        let mut galaxies = parse_galaxies(&input);
        shift(&mut galaxies, 99);
        assert_eq!(distances_sum(&galaxies), 8410);
    }
}
