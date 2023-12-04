advent_of_code::solution!(4);

fn get_count(line: &str)-> usize {
    let (_, u) = line.split_once(":").unwrap();
    let (before, after) = u.split_once("|").unwrap();
    let a = before.split_ascii_whitespace().map(fast_string_parse).collect::<Vec<_>>();
    after.split_ascii_whitespace().map(fast_string_parse).fold(0, |i, x| if a.contains(&x) { i + 1 } else { i })
}

fn fast_string_parse(x: &str) -> u32 {
    let mut ans = 0;
    for i in x.bytes() {
        ans = ans * 10 + (i - b'0') as u32;
    }
    ans
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().map(|x| {
        let c = get_count(x);
        let v = if c > 0 { 1 << (c - 1) } else { 0 };
        v
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut count  = vec![1u32; lines.len()];
    for i in 0..lines.len() {
        let c = get_count(lines[i]);
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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
