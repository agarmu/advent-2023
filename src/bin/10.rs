use itertools::Itertools;

advent_of_code::solution!(10);

type Map = Vec<Vec<u8>>;

#[inline(always)]
pub fn parse(input: &str) -> (Map, (usize, usize)) {
    let res = input.lines().map(|x| x.bytes().collect_vec()).collect_vec();
    let loc = res
        .iter()
        .enumerate()
        .filter_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_, v)| **v == b'S')
                .map(|(x, _)| (y as usize, x as usize))
        })
        .next()
        .unwrap();
    (res, loc)
}

enum Direction {
    Left,
    Up,
    Right,
    Down,
}

pub fn find_loop(map: Map, start: (usize, usize)) -> Vec<(usize, usize)> {
    use Direction::*;
    [Left, Right, Up, Down]
        .into_iter()
        .filter_map(|mut dir| {
            // we leave as a vector instead of a set because
            // even though lookup time is O(n) the list is sufficiently small
            // that this is better than a second allocation
            let mut visited = Vec::new();
            let (mut y, mut x) = start;
            while !visited.contains(&(y, x)) {
                visited.push((y, x));
                match dir {
                    Right => {
                        x += 1;
                        let val = map[y][x];
                        match val {
                            b'J' => dir = Up,
                            b'7' => dir = Down,
                            b'-' => continue,
                            _ => break,
                        }
                    }
                    Left => {
                        x -= 1;
                        let val = map[y][x];
                        match val {
                            b'L' => dir = Up,
                            b'F' => dir = Down,
                            b'-' => continue,
                            _ => break,
                        }
                    }
                    Down => {
                        y += 1;
                        let val = map[y][x];
                        match val {
                            b'J' => dir = Left,
                            b'L' => dir = Right,
                            b'|' => continue,
                            _ => break,
                        }
                    }
                    Up => {
                        y -= 1;
                        let val = map[y][x];
                        match val {
                            b'7' => dir = Left,
                            b'F' => dir = Right,
                            b'|' => continue,
                            _ => break,
                        }
                    }
                }
            }
            if (y, x) == start {
                visited.push(start);
                Some(visited)
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[inline(always)]
pub fn part_one(input: &str) -> Option<usize> {
    let (map, start) = parse(input);
    let result = find_loop(map, start);
    Some(result.len() / 2)
}

pub fn shoelace(vals: &[(i64, i64)]) -> i64 {
    let mut total = 0;
    for i in 0..vals.len() {
        let a = vals[i];
        let b = vals[(i + 1) % vals.len()];
        total += a.0 * b.1 - b.0 * a.1;
    }
    total.abs()
}

pub fn area(x: &[(i64, i64)]) -> i64 {
    /* pick's thm */
    shoelace(x) / 2 - (x.len() as i64) / 2 + 1
}

pub fn part_two(input: &str) -> Option<i64> {
    let (map, start) = parse(input);
    let result = find_loop(map, start)
        .into_iter()
        .map(|(i, j)| (i as i64, j as i64))
        .collect_vec();
    Some(area(&result))
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
