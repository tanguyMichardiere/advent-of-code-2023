use itertools::Itertools;

use crate::regex;

fn is_five_of_a_kind(sorted_cards: (u8, u8, u8, u8, u8)) -> bool {
    sorted_cards.0 == sorted_cards.1
        && sorted_cards.1 == sorted_cards.2
        && sorted_cards.2 == sorted_cards.3
        && sorted_cards.3 == sorted_cards.4
}

fn is_four_of_a_kind(sorted_cards: (u8, u8, u8, u8, u8)) -> bool {
    (
        // AAAAB
        sorted_cards.0 == sorted_cards.1
            && sorted_cards.1 == sorted_cards.2
            && sorted_cards.2 == sorted_cards.3
            && sorted_cards.3 < sorted_cards.4
    ) || (
        // ABBBB
        sorted_cards.0 < sorted_cards.1
            && sorted_cards.1 == sorted_cards.2
            && sorted_cards.2 == sorted_cards.3
            && sorted_cards.3 == sorted_cards.4
    )
}

fn is_full_house(sorted_cards: (u8, u8, u8, u8, u8)) -> bool {
    (
        // AAABB
        sorted_cards.0 == sorted_cards.1
            && sorted_cards.1 == sorted_cards.2
            && sorted_cards.2 < sorted_cards.3
            && sorted_cards.3 == sorted_cards.4
    ) || (
        // AABBB
        sorted_cards.0 == sorted_cards.1
            && sorted_cards.1 < sorted_cards.2
            && sorted_cards.2 == sorted_cards.3
            && sorted_cards.3 == sorted_cards.4
    )
}

fn is_three_of_a_kind(sorted_cards: (u8, u8, u8, u8, u8)) -> bool {
    (
        // AAABC
        sorted_cards.0 == sorted_cards.1
            && sorted_cards.1 == sorted_cards.2
            && sorted_cards.2 < sorted_cards.3
            && sorted_cards.3 < sorted_cards.4
    ) || (
        // ABBBC
        sorted_cards.0 < sorted_cards.1
            && sorted_cards.1 == sorted_cards.2
            && sorted_cards.2 == sorted_cards.3
            && sorted_cards.3 < sorted_cards.4
    ) || (
        // ABCCC
        sorted_cards.0 < sorted_cards.1
            && sorted_cards.1 < sorted_cards.2
            && sorted_cards.2 == sorted_cards.3
            && sorted_cards.3 == sorted_cards.4
    )
}

fn is_two_pair(sorted_cards: (u8, u8, u8, u8, u8)) -> bool {
    (
        // AABBC
        sorted_cards.0 == sorted_cards.1
            && sorted_cards.1 < sorted_cards.2
            && sorted_cards.2 == sorted_cards.3
            && sorted_cards.3 < sorted_cards.4
    ) || (
        // AABCC
        sorted_cards.0 == sorted_cards.1
            && sorted_cards.1 < sorted_cards.2
            && sorted_cards.2 < sorted_cards.3
            && sorted_cards.3 == sorted_cards.4
    ) || (
        // ABBCC
        sorted_cards.0 < sorted_cards.1
            && sorted_cards.1 == sorted_cards.2
            && sorted_cards.2 < sorted_cards.3
            && sorted_cards.3 == sorted_cards.4
    )
}

fn is_one_pair(sorted_cards: (u8, u8, u8, u8, u8)) -> bool {
    (
        // AABCD
        sorted_cards.0 == sorted_cards.1
            && sorted_cards.1 < sorted_cards.2
            && sorted_cards.2 < sorted_cards.3
            && sorted_cards.3 < sorted_cards.4
    ) || (
        // ABBCD
        sorted_cards.0 < sorted_cards.1
            && sorted_cards.1 == sorted_cards.2
            && sorted_cards.2 < sorted_cards.3
            && sorted_cards.3 < sorted_cards.4
    ) || (
        // ABCCD
        sorted_cards.0 < sorted_cards.1
            && sorted_cards.1 < sorted_cards.2
            && sorted_cards.2 == sorted_cards.3
            && sorted_cards.3 < sorted_cards.4
    ) || (
        // ABCDD
        sorted_cards.0 < sorted_cards.1
            && sorted_cards.1 < sorted_cards.2
            && sorted_cards.2 < sorted_cards.3
            && sorted_cards.3 == sorted_cards.4
    )
}

fn is_high_card(sorted_cards: (u8, u8, u8, u8, u8)) -> bool {
    sorted_cards.0 < sorted_cards.1
        && sorted_cards.1 < sorted_cards.2
        && sorted_cards.2 < sorted_cards.3
        && sorted_cards.3 < sorted_cards.4
}

fn hand_type(cards: (u8, u8, u8, u8, u8)) -> u8 {
    let sorted_cards = [cards.0, cards.1, cards.2, cards.3, cards.4]
        .into_iter()
        .sorted()
        .collect_tuple::<(_, _, _, _, _)>()
        .unwrap();

    if is_five_of_a_kind(sorted_cards) {
        6
    } else if is_four_of_a_kind(sorted_cards) {
        5
    } else if is_full_house(sorted_cards) {
        4
    } else if is_three_of_a_kind(sorted_cards) {
        3
    } else if is_two_pair(sorted_cards) {
        2
    } else if is_one_pair(sorted_cards) {
        1
    } else if is_high_card(sorted_cards) {
        0
    } else {
        unreachable!()
    }
}

pub fn part_one(input: &str) -> usize {
    regex!(r"(?P<hand>[AKQJT98765432]{5}) (?P<bid>\d+)")
        .captures_iter(input)
        .map(|caps| {
            (
                caps["hand"]
                    .chars()
                    .map(|char| match char {
                        'A' => 14,
                        'K' => 13,
                        'Q' => 12,
                        'J' => 11,
                        'T' => 10,
                        '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' => {
                            char.to_digit(10).unwrap() as u8
                        }
                        _ => unreachable!(),
                    })
                    .collect_tuple::<(_, _, _, _, _)>()
                    .unwrap(),
                caps["bid"].parse::<usize>().unwrap(),
            )
        })
        .sorted_by_cached_key(|(cards, _)| {
            (
                hand_type(*cards),
                cards.0,
                cards.1,
                cards.2,
                cards.3,
                cards.4,
            )
        })
        .enumerate()
        .map(|(rank, (_, bid))| bid * (rank + 1))
        .sum()
}

fn hand_type_with_jokers(cards: (u8, u8, u8, u8, u8)) -> u8 {
    [cards.0, cards.1, cards.2, cards.3, cards.4]
        .into_iter()
        .filter(|card| card != &1)
        .chain([1])
        .map(|card| {
            (
                if cards.0 == 1 { card } else { cards.0 },
                if cards.1 == 1 { card } else { cards.1 },
                if cards.2 == 1 { card } else { cards.2 },
                if cards.3 == 1 { card } else { cards.3 },
                if cards.4 == 1 { card } else { cards.4 },
            )
        })
        .map(hand_type)
        .max()
        .unwrap()
}

pub fn part_two(input: &str) -> usize {
    regex!(r"(?P<hand>[AKQJT98765432]{5}) (?P<bid>\d+)")
        .captures_iter(input)
        .map(|caps| {
            (
                caps["hand"]
                    .chars()
                    .map(|char| match char {
                        'A' => 13,
                        'K' => 12,
                        'Q' => 11,
                        'T' => 10,
                        '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' => {
                            char.to_digit(10).unwrap() as u8
                        }
                        'J' => 1,
                        _ => unreachable!(),
                    })
                    .collect_tuple::<(_, _, _, _, _)>()
                    .unwrap(),
                caps["bid"].parse::<usize>().unwrap(),
            )
        })
        .sorted_by_cached_key(|(cards, _)| {
            (
                hand_type_with_jokers(*cards),
                cards.0,
                cards.1,
                cards.2,
                cards.3,
                cards.4,
            )
        })
        .enumerate()
        .map(|(rank, (_, bid))| bid * (rank + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_to_string("examples/07/1").unwrap();
        assert_eq!(part_one(&input), 6440);
    }

    #[test]
    fn test_part_two() {
        let input = read_to_string("examples/07/1").unwrap();
        assert_eq!(part_two(&input), 5905);
    }
}
