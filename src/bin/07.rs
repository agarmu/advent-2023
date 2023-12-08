use std::{
    cmp::Ordering,
    collections::HashMap,
};

use itertools::Itertools;

advent_of_code::solution!(7);

fn parse_char(x: u8, jvalue: u8) -> u8 {
    match x {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => jvalue,
        b'T' => 10,
        u => u - b'0',
    }
}
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum GameRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone)]
pub struct Game<const N: usize> {
    values: Vec<u8>,
    bid: usize,
    rank: GameRank,
}
pub fn get_rank(x: &[usize]) -> GameRank {
    use GameRank::*;
    match x.len() {
        1 => FiveOfAKind, // five of a kind
        2 => {
            if x[0] == 4 {
                FourOfAKind
            } else {
                FullHouse
            }
        }
        3 => {
            if x[0] == 3 {
                ThreeOfAKind
            } else {
                TwoPair
            }
        }
        4 => OnePair,
        5 => HighCard,
        _ => unreachable!(),
    }
}
impl<const N: usize> Game<N> {
    fn parse(x: &str) -> Self {
        let jvalue = match N {
            1 => 11,
            2 => 1,
            _ => unreachable!()
        };
        let (a, b) = x.trim().split_once(" ").unwrap();
        let values = a.bytes().map(|x| parse_char(x, jvalue)).collect_vec();
        let bid = b.parse().unwrap();
        let mut countmap = HashMap::<u8, usize>::with_capacity(5);
        for x in &values {
            *countmap.entry(*x).or_insert(0) += 1;
        }
        let jokercount = countmap.remove(&1).unwrap_or(0);
        let rank = if jokercount == 5 {
            GameRank::FiveOfAKind
        } else {
            let mut counts = countmap
                .into_iter()
                .map(|(_, x)| x)
                .sorted()
                .rev()
                .collect_vec();
            counts[0] += jokercount;
            get_rank(&counts)
        };
        Self { values, rank, bid }
    }

    fn cmp_vals(&self, other: &Self) -> Ordering {
        for i in 0..5 {
            match u8::cmp(&self.values[i], &other.values[i]) {
                Ordering::Equal => continue,
                x => return x,
            }
        }
        Ordering::Equal
    }
}
impl<const N: usize> PartialEq for Game<N> {
    fn eq(&self, other: &Self) -> bool {
        self.values == other.values
    }
}
impl<const N: usize> Eq for Game<N> {}
impl<const N: usize> PartialOrd for Game<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match u8::cmp(&(self.rank as u8), &(other.rank as u8)) {
            Ordering::Equal => Some(self.cmp_vals(other)),
            x => Some(x),
        }
    }
}
impl<const N: usize> Ord for Game<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        match u8::cmp(&(self.rank as u8), &(other.rank as u8)) {
            Ordering::Equal => self.cmp_vals(other),
            x => x,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> { solve::<1>(input) }
pub fn part_two(input: &str) -> Option<usize> { solve::<2>(input) }

pub fn solve<const N: usize>(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(Game::<N>::parse)
            .sorted()
            .enumerate()
            .map(|(i, v)| (i + 1) * v.bid)
            .sum()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
