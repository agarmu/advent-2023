advent_of_code::solution!(1);

const R: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
pub fn compute1(input: &str) -> u32 {
    let x = input.as_bytes();
    let mut fd = 0;
    let mut ld = 0;
    for v in x.iter() {
        if v.is_ascii_digit() {
            fd = (v - b'0') as u32;
            break;
        }
    }
    for i in (0..input.len()).rev() {
        if x[i].is_ascii_digit() {
            ld = (x[i] - b'0') as u32;
            break;
        }
    }
    fd * 10 + ld
}
pub fn compute2(input: &str) -> u32 {
    let x = input.as_bytes();
    let mut fd = 0;
    let mut ld = 0;
    'outer: for i in 0..input.len() {
        if x[i].is_ascii_digit() {
            fd = (x[i] - b'0') as u32;
            break;
        }
        for (k, r) in R.iter().enumerate() {
            if i + r.len() <= input.len() && input[i..].starts_with(r) {
                fd = k as u32 + 1;
                break 'outer;
            }
        }
    }
    'outer: for i in (0..input.len()).rev() {
        if x[i].is_ascii_digit() {
            ld = (x[i] - b'0') as u32;
            break;
        }
        for (k, r) in R.iter().enumerate() {
            if i + r.len() <= input.len() && input[i..].starts_with(r) {
                ld = k as u32 + 1;
                break 'outer;
            }
        }
    }
    fd * 10 + ld
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(compute1).sum())
}
pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(compute2).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
