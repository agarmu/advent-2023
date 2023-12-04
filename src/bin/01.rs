advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .map(|x| {
            let mut q = x.bytes()
                .filter(|x| x >= &b'0' && x <= &b'9')
                .map(|x| (x - b'0') as u32);
            let fd = q.next().unwrap();
            let ld = q.last().unwrap_or(fd);
            fd * 10 + ld
        }).sum())
}
const R: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
];
pub fn make_stack(input: &str, res: &mut Vec<u32>) {
    res.clear();
    let x = input.as_bytes();
    for i in 0..input.len() {
        for (k, r) in R.iter().enumerate() {
            if i + r.len() <= input.len() {
               if input[i..].starts_with(r) {
                    res.push(k as u32 + 1);
                    continue;
               }
            }
        }
        if x[i] >= b'0' && x[i] <= b'9' {
            res.push((x[i] - b'0') as u32);
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut res: Vec<u32> = Vec::with_capacity(16);
    Some(input
        .lines()
        .map(|x| {
            make_stack(x, &mut res);
            res[0] * 10 + res[res.len() - 1]
        }).sum())
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
