advent_of_code::solution!(6);
const DELTA: f32 = 0.001;
pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .strip_prefix("Time:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.bytes().fold(0.0f32, |a, x| 10.0 * a + (x - b'0') as f32));
    let distances = lines
        .next()
        .unwrap()
        .strip_prefix("Distance:")
        .unwrap()
        .split_ascii_whitespace()
        .map(|x| x.bytes().fold(0.0f32, |a, x| 10.0 * a + (x - b'0') as f32));
    Some(
        times
            .zip(distances)
            .map(|(x, y)| do_calculation(x, y))
            .product(),
    )
}

#[inline(always)]
fn do_calculation(time: f32, dist: f32) -> u32 {
    let disc = time * time - 4.0 * dist;
    let disc = disc.sqrt() / 2.0;
    let min = (f32::max(0.0, time / 2.0 - disc) + DELTA + 0.5) as u32;
    let max = (f32::min(time, time / 2.0 + disc) - DELTA) as u32;
    max - min + 1
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let time = lines
        .next()
        .unwrap()
        .bytes()
        .filter(|x| x.is_ascii_digit())
        .fold(0.0f32, |a, x| 10.0 * a + (x - b'0') as f32);
    let dist = lines
        .next()
        .unwrap()
        .bytes()
        .filter(|x| x.is_ascii_digit())
        .fold(0.0f32, |a, x| 10.0 * a + (x - b'0') as f32);
    Some(do_calculation(time, dist))
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
