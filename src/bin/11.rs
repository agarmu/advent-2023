use std::collections::LinkedList;

use itertools::Itertools;

advent_of_code::solution!(11);

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
enum Point {
    Galaxy,
    Empty,
}
impl Point {
    fn parse(x: u8) -> Self {
        if x == b'#' {
            Self::Galaxy
        } else {
            Self::Empty
        }
    }
}
type Map = Vec<Vec<Point>>;
fn parse_map(input: &str) -> Map {
    input
        .lines()
        .map(|x| x.bytes().map(Point::parse).collect_vec())
        .collect_vec()
}
fn find_galaxies(map: &Map) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for (y, line) in map.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val == Point::Galaxy {
                res.push((y, x))
            }
        }
    }
    res
}

pub fn expand_range<const F: usize>(orig: &[usize], count: usize) -> Vec<usize> {
    let mut new = orig.to_owned();
    let mut i = 0usize;
    let mut upto = 0usize;
    loop {
        if upto >= count || i >= orig.len() {
            break;
        }
        let delta = orig[i] - upto;
        if delta > 1 {
            let expansion = (delta - 1) * (F - 1);
            if delta > 0 {
                new[i..].iter_mut().for_each(|x| *x += expansion);
            }
        }
        upto = orig[i];
        i += 1;
    }
    new
}

fn find_min_distance<const F: usize>(map: Map) -> usize {
    let mut galaxies = find_galaxies(&map);
    // expansion
    let rows = galaxies.iter().map(|x| x.0).sorted().unique().collect_vec();
    let cols = galaxies.iter().map(|x| x.1).sorted().unique().collect_vec();
    // copies
    let new_rows = expand_range::<F>(&rows, map.len());
    let new_cols = expand_range::<F>(&cols, map[0].len());
    // expand the galaxies
    for galaxy in galaxies.iter_mut() {
        let newy = new_rows[rows.iter().position(|x| *x == galaxy.0).unwrap()];
        let newx = new_cols[cols.iter().position(|x| *x == galaxy.1).unwrap()];
        *galaxy = (newy, newx);
    }
    let mut total = 0;
    for i in 0..(galaxies.len() - 1) {
        let a = galaxies[i];
        for j in (i + 1)..(galaxies.len()) {
            let b = galaxies[j];
            let dist = usize::abs_diff(b.1, a.1) + usize::abs_diff(b.0, a.0);
            total += dist;
        }
    }
    total
}

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse_map(input);
    Some(find_min_distance::<2>(map))
}

pub fn part_two(input: &str) -> Option<usize> {
    let map = parse_map(input);
    Some(find_min_distance::<1000000>(map))
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
