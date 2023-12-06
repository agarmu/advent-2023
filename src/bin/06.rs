advent_of_code::solution!(6);
const DELTA: f64 = 0.001;
pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.bytes().fold(0.0f64, |a, x| 10.0 * a + (x - b'0') as f64));
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.bytes().fold(0.0f64, |a, x| 10.0 * a + (x - b'0') as f64));
    let mut res = 1;
    for (time, dist) in times.zip(distances) {
        let disc = time * time - 4.0 * dist;
        let disc = disc.sqrt() / 2.0;
        let min = (f64::max(0.0, time / 2.0 - disc) + DELTA).ceil() as u32;
        let max = (f64::min(time, time / 2.0 + disc) - DELTA).floor() as u32;
        res *= max - min + 1;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .bytes()
        .filter(|x| !x.is_ascii_whitespace())
        .fold(0.0f64, |a, x| 10.0 * a + (x - b'0') as f64);
    let dist = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .bytes()
        .filter(|x| !x.is_ascii_whitespace())
        .fold(0.0f64, |a, x| 10.0 * a + (x - b'0') as f64);
    let disc = time * time - 4.0 * dist;
    if disc <= 0.0 {
        return None;
    }
    let disc = disc.sqrt() / 2.0;
    let fmin = f64::max(0.0, time / 2.0 - disc);
    let fmax = f64::min(time, time / 2.0 + disc);
    let min = (fmin + DELTA).ceil() as u32;
    let max = (fmax - DELTA).floor() as u32;
    if max <= min {
        return None;
    }
    Some(max - min + 1)
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
