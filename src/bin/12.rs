use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator, IntoParallelIterator};

advent_of_code::solution!(12);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Spring {
    Working, Damaged, Unknown
}

#[derive(Debug, Clone)]
struct Record {
    springs: Vec<Spring>,
    counts :Vec<usize>
}

impl Record {
    fn parse<const N: usize>(x: &str) -> Self {
        let (f, l) = x.split_once(" ").unwrap();
        let mut springs = f.bytes().map(Spring::parse).collect_vec();
        let mut counts = l.split(",").filter_map(|x| x.parse().ok()).collect_vec();
        if N > 1 {
            springs.push(Spring::Unknown);
            springs = springs.repeat(5);
            counts = counts.repeat(5);
        }
        Self {springs, counts}
    }

    fn is_valid(&self) -> bool {
        self
            .springs
            .iter()
            .group_by(|x| *x)
            .into_iter()
            .filter_map(|(x, g)| {
                if *x == Spring::Damaged { Some(g.count()) } else { None }
            }).eq(self.counts.iter().copied())
    }

    fn count_valid_arrangements(mut self) -> u64 {
        // we add an extra value at the end to make recursion simpler
        // (since we don't care about working springs at all-- just use that)
        self.springs.push(Spring::Working);
        let mut memo = vec![vec![None; self.springs.len()]; self.counts.len()];
        Record::memoized_counting(&self.springs, &self.counts, &mut memo)
    }
    fn memoized_counting(springs: &[Spring], counts: &[usize], memo: &mut [Vec<Option<u64>>]) -> u64 {
        if counts.is_empty() {
            if springs.contains(&Spring::Damaged) {
                0
            } else {
                1
            }
        } else if springs.len() < counts.iter().sum::<usize>() + counts.len() {
            return 0;
        } else if let Some(c) = memo[counts.len() - 1][springs.len() - 1] {
            c
        } else {
            let mut arrangements = 0;
            if springs[0] != Spring::Damaged {
                arrangements += Record::memoized_counting(&springs[1..], counts, memo);
            }
            let next_grp = counts[0];
            if !springs[..next_grp].contains(&Spring::Working) && springs[next_grp] != Spring::Damaged {
                arrangements += Record::memoized_counting(&springs[(next_grp + 1)..], &counts[1..], memo);
            }
            memo[counts.len() - 1][springs.len() - 1] = Some(arrangements);
            arrangements
        }
    }

}

impl Spring {
    fn parse(x: u8) -> Self {
        match x {
            b'.' => Self::Working,
            b'?' => Self::Unknown,
            b'#' => Self::Damaged,
            _ => unreachable!()
        }
    }
}


pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(Record::parse::<1>)
        .into_iter()
        .map(|x| Record::count_valid_arrangements(x))
        .sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
