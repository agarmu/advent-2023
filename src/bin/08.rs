use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Debug, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

impl Direction {
    fn parse(x: u8) -> Self {
        match x {
            b'R' => Self::Right,
            _ => Self::Left,
        }
    }
}

#[inline(always)]
const fn conv_dig(x: u8) -> u32 {
    (x - b'A') as u32
}

#[inline(always)]
const fn conv(x: &[u8]) -> u32 {
    (conv_dig(x[0]) << 16) + (conv_dig(x[1]) << 8) + conv_dig(x[2])
}

fn parse_rule(x: &str) -> (u32, (u32, u32)) {
    let y = x.as_bytes();
    let f = conv(&y[0..3]);
    let l = conv(&y[7..10]);
    let r = conv(&y[12..15]);
    (f, (l, r))
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut x = input.lines();
    let directions = x
        .next()
        .unwrap()
        .bytes()
        .map(Direction::parse)
        .collect_vec();
    let mappings = x.skip(1).map(parse_rule).collect::<HashMap<_, _>>();
    Some(calculate_distance::<NonGhostly>(
        AAA,
        &directions,
        &mappings,
    ))
}

pub trait Terminator {
    fn is_terminal(x: u32) -> bool;
}

struct NonGhostly;

const AAA: u32 = conv(&[b'A', b'A', b'A']);
const ZZZ: u32 = conv(&[b'Z', b'Z', b'Z']);
impl Terminator for NonGhostly {
    #[inline(always)]
    fn is_terminal(x: u32) -> bool {
        x == ZZZ
    }
}
struct Ghostly;
impl Terminator for Ghostly {
    #[inline(always)]
    fn is_terminal(x: u32) -> bool {
        x & 0b11111111 == conv_dig(b'Z')
    }
}

fn calculate_distance<T: Terminator>(
    start: u32,
    d: &[Direction],
    lookup: &HashMap<u32, (u32, u32)>,
) -> usize {
    let mut cur = start;
    let mut count = 0;
    let mut dir_provider = d.iter().cycle();
    while !T::is_terminal(cur) {
        let v = lookup.get(&cur).unwrap();
        cur = match dir_provider.next().unwrap() {
            Direction::Left => v.0,
            Direction::Right => v.1,
        };
        count += 1;
    }
    count
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut x = input.lines();
    let directions = x
        .next()
        .unwrap()
        .bytes()
        .map(Direction::parse)
        .collect_vec();
    let mappings = x.skip(1).map(parse_rule).collect::<HashMap<_, _>>();
    Some(
        mappings
            .iter()
            .map(|x| *x.0)
            .filter(|x| x & 0b11111111 == conv_dig(b'A'))
            .map(|x| calculate_distance::<Ghostly>(x, &directions, &mappings))
            .fold(directions.len(), lcm),
    )
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

fn gcd(x: usize, y: usize) -> usize {
    let min;
    let max;
    if x < y {
        min = x;
        max = y;
    } else {
        min = y;
        max = x;
    }
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
