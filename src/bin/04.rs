use std::collections::HashSet;

advent_of_code::solution!(4);

pub struct Card {
    number: u32,
    winners: u32
}


pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|x| {
        let mut u = x.split(":").skip(1).next().unwrap().split("|");
        let iter = u.next().unwrap().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap());
        let a = HashSet::<_>::from_iter(iter);
        let c = u.next().unwrap().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).filter(|x| a.contains(x)).count();
        let v = if c > 0 { 1 << (c - 1) } else { 0 };
        v
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().enumerate().collect::<Vec<_>>();
    let mut count  = vec![1u32; lines.len()];
    for (i, x) in lines {
        let mut u = x.split(":").skip(1).next().unwrap().split("|");
        let iter = u.next().unwrap().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap());
        let a = HashSet::<_>::from_iter(iter);
        let c = u.next().unwrap().split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).filter(|x| a.contains(x)).count();
        // the next several get incremented by value of count[i];
        let amt = count[i];
        for j in 1..=c {
            count[i + j] += amt;
        }
    }
    Some(count.iter().sum())
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
