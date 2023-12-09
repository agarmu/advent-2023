use itertools::Itertools;

advent_of_code::solution!(9);

pub fn calculate_differences(x: &mut [i64]) -> &mut [i64] {
    if x.len() == 0 {
        panic!("???")
    }
    if x.len() == 1 {
        x
    } else {
        let end = x.len() - 1;
        for i in 0..end {
            x[i] = x[i + 1] - x[i];
        }
        &mut x[0..end]
    }
}

pub fn calculate_next_value(x: &mut [i64]) -> i64 {
    if x.len() == 1 {
        return x[0];
    } // base case
    let differences = calculate_differences(x);
    if differences.iter().all(|x| *x == 0) {
        return x[x.len() - 1]; // all diffs 0 so constant
    }
    let next = calculate_next_value(differences);
    let r = x[x.len() - 1] + next;
    r
}

pub fn calculate_prev_value(x: &mut [i64]) -> i64 {
    let fst = x[0];
    if x.len() == 1 {
        return fst;
    } // base case
    let differences = calculate_differences(x);
    if differences.iter().all(|x| *x == 0) {
        return fst; // all diffs 0 so constant
    }
    let prev = calculate_prev_value(differences);
    fst - prev
}


pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|x| {
                    let mut v = x.split_ascii_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect_vec();
                    let r = calculate_next_value(&mut v);
                    r
            })
            .sum(),
    )
}
pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|x| {
                    let mut v = x.split_ascii_whitespace()
                        .map(|x| x.parse::<i64>().unwrap())
                        .collect_vec();
                    calculate_prev_value(&mut v)
            })
            .sum(),
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
