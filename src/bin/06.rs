advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap());
    let mut res = 1;
    for (time, dist) in times.zip(distances) {
        let mut q = 0;
        for accel_time in 1..time {
            if dist < (time - accel_time) * accel_time {
                q += 1;
            }
        }
        res *= q;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>();
    let dist = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .chars()
        .filter(|x| x.is_digit(10))
        .collect::<String>();
    let time = time.parse().unwrap();
    let dist: u64 = dist.parse().unwrap();
    let mut q = 0;
    for accel_time in 1..time {
        if dist < (time - accel_time) * accel_time {
            q += 1;
        }
    }
    Some(q)
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
