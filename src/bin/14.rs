use std::collections::{HashSet, HashMap};

use itertools::Itertools;

advent_of_code::solution!(14);

fn parse_map<T>(x: &str) -> Vec<Vec<T>>
where
    T: From<u8>,
{
    x.lines()
        .map(|y| y.bytes().map(|u| T::from(u)).collect_vec())
        .collect_vec()
}

fn print_map(x: &Vec<Vec<u8>>) {
    for line in x {
        println!("{}", String::from_utf8(line.to_owned()).unwrap());
    }
}

pub fn up(data: &mut Vec<Vec<u8>>, height: usize, width: usize) {
    for x in 0..width {
        for mut y in 0..height {
            if data[y][x] != b'O' { continue; }
            data[y][x] = b'.';
            while y < height && data[y][x] == b'.' {
                y = y.wrapping_sub(1);
            }
            y = y.wrapping_add(1);
            data[y][x] = b'O';
        }
    }
}

pub fn down(data: &mut Vec<Vec<u8>>, height: usize, width: usize) {
    for x in 0..width {
        for mut y in 0..height {
            if data[y][x] != b'O' { continue; }
            data[y][x] = b'.';
            while y < height && data[y][x] == b'.' {
                y += 1;
            }
            y-= 1;
            data[y][x] = b'O';
        }
    }
}

pub fn left(data: &mut Vec<Vec<u8>>, height: usize, width: usize) {
    for y in 0..height {
        for mut x in 0..width {
            if data[y][x] != b'O' { continue; }
            data[y][x] = b'.';
            while x < width && data[y][x] == b'.' {
                x = x.wrapping_sub(1);
            }
            x = x.wrapping_add(1);
            data[y][x] = b'O';
        }
    }
}

pub fn right(data: &mut Vec<Vec<u8>>, height: usize, width: usize) {
    for y in 0..height {
        for mut x in 0..width {
            if data[y][x] != b'O' { continue; }
            data[y][x] = b'.';
            while x < width && data[y][x] == b'.' {
                x += 1;
            }
            x -= 1;
            data[y][x] = b'O';
        }
    }
}
pub fn score(data: &Vec<Vec<u8>>) -> usize {
    data.iter().enumerate().map(|(y, l)| {
        l.iter().filter(|x| **x == b'O').count() * (data.len() - y)
    }).sum()
}


pub fn part_one(input: &str) -> Option<usize> {
    let mut data = parse_map::<u8>(input);
    let width = data[0].len();
    let height = data.len();
    up(&mut data, height, width);
    Some(score(&data))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut data = parse_map::<u8>(input);
    let mut states = HashMap::<Vec<Vec<u8>>, usize>::new();
    let mut scores = Vec::with_capacity(1000);
    let mut i = 0usize;
    let width = data[0].len();
    let height = data.len();
    while i < 1000000000 {
        if states.contains_key(&data) { break; } // previously happened!
        for f in [up, right, down, left]  {
            f(&mut data, height, width);
        }
        states.insert(data.clone(), i);
        scores.push(score(&data));
        i+= 1;
    }
    // find loop length
    let i_s = states.get(&data).unwrap();
    let loop_len = (i - i_s);
    let ip = (1000000000 - i) % loop_len + i_s;
    println!("{}, {} (len of {})", i, i_s, loop_len);
    println!("{:?}", scores);
    Some(scores[ip])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
