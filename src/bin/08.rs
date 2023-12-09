use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Copy, Clone)]
enum Direction { Left, Right }

impl Direction {
    fn parse(x: u8) -> Self {
        match x {
            b'R' => Self::Right,
            _ => Self::Left
        }
    }
}

fn parse_rule(x: &str) -> (&str, (&str, &str)) {
    let y = x.as_bytes();
    let f = &y[0..3];
    let l = &y[7..10];
    let r = &y[12..15];
    unsafe {
        let f = std::str::from_utf8_unchecked(f);
        let l = std::str::from_utf8_unchecked(l);
        let r = std::str::from_utf8_unchecked(r);
        (f, (l, r))
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut x = input.lines();
    let directions = x.next().unwrap().bytes().map(Direction::parse).collect_vec();
    let mappings = x.skip(1).map(parse_rule).collect::<HashMap<_, _>>();
    Some(calculate_distance::<NonGhostly>("AAA", &directions, &mappings))
}

pub trait Terminator {
    fn is_terminal(x: &str) -> bool;
}

struct NonGhostly;

impl Terminator for NonGhostly {
    #[inline(always)]
    fn is_terminal(x: &str) -> bool {
        x == "ZZZ"
    }
}
struct Ghostly;
impl Terminator for Ghostly {
    #[inline(always)]
    fn is_terminal(x: &str) -> bool {
        x.ends_with('Z')
    }
}

fn calculate_distance<T: Terminator>(start: &str, d: &[Direction], lookup: &HashMap<&str, (&str, &str)>) -> usize {
    let mut cur = start;
    let mut count = 0;
    let mut dir_provider = d.iter().cycle();
    while !T::is_terminal(cur) {
        let v = lookup.get(cur).unwrap();
        cur = match dir_provider.next().unwrap() {
            Direction::Left => v.0,
            Direction::Right => v.1
        };
        count += 1;
    }
    count
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut x = input.lines();
    let directions = x.next().unwrap().bytes().map(Direction::parse).collect_vec();
    let mappings = x.skip(1).map(parse_rule).collect::<HashMap<_, _>>();
    mappings
        .iter()
        .map(|x| *x.0)
        .filter(|x| x.ends_with('A'))
        .map(|x| calculate_distance::<Ghostly>(x, &directions, &mappings))
        .reduce(lcm)
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

fn gcd(x: usize, y: usize) -> usize {
    let min;
    let max;
    if x < y { min = x; max = y; } else { min = y; max = x; }
    if min == 0 {
        max
    } else {
        gcd(min, max % min)
    }
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
