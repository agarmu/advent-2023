advent_of_code::solution!(3);

struct PrePost<T: Iterator> {
    x: T,
    pre: Option<T::Item>,
    post: Option<T::Item>,
}

impl<T: Iterator> Iterator for PrePost<T> {
    type Item = T::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.pre.is_none() {
            match self.x.next() {
                None => {
                    let mut x = None;
                    std::mem::swap(&mut x, &mut self.post);
                    x
                }
                x => x
            }
        } else {
            let mut x = None;
            std::mem::swap(&mut x, &mut self.pre);
            x
        }
    }
}

#[inline(always)]
fn parse(input: &str) -> Vec<Vec<u8>> {
    let mut data = input.lines().map(|x| {
       PrePost { x: PrePost {x: x.bytes(), pre:Some(b'.'), post: Some(b'.')}, pre: Some(b'.'), post: Some(b'.')}.collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    data.insert(0, vec![b'.'; data[0].len()]);
    data.push(vec![b'.'; data[0].len()]);
    data
}

fn check_adjacents(i: usize, js: usize, je: usize, data: &Vec<Vec<u8>>) -> bool {
    // top
    for j in js..=je {
        let q = data[i - 1][j];
        if q.is_ascii_digit() || q == b'.' { continue; } else { return true; }
    }
    // bottom
    for j in js..=je {
        let q = data[i + 1][j];
        if q.is_ascii_digit() || q == b'.' { continue; } else { return true; }
    }
    // left
    for i in (i-1)..=(i+1) {
        let q = data[i][js - 1];
        if q.is_ascii_digit() || q == b'.' { continue; } else { return true; }
    }
    // right
    for i in (i-1)..=(i+1) {
        let q = data[i][je + 1];
        if q.is_ascii_digit() || q == b'.' { continue; } else { return true; }
    }
    false
}

fn check_lr(i: usize, j: usize, data: &Vec<Vec<u8>>, numbers: &mut Vec<u32>) {
    if data[i][j].is_ascii_digit() {
        let mut num = (data[i][j] - b'0') as u32;
        let mut k = j - 1;
        let mut multiplier = 10u32;
        // search left!
        while data[i][k].is_ascii_digit() {
            num = (data[i][k] - b'0') as u32 * multiplier + num;
            k -= 1;
            multiplier *= 10;
        }
        // search right!
        let mut k = j + 1;
        while data[i][k].is_ascii_digit() {
            num = num * 10 + (data[i][k] - b'0') as u32;
            k += 1;
        }
        // add number to numbers
        numbers.push(num);
    } else {
        check_left(i, j - 1, data, numbers);
        check_right(i, j + 1, data, numbers);
    }
}

fn check_left(i: usize, j: usize, data: &Vec<Vec<u8>>, numbers: &mut Vec<u32>) {
    if data[i][j].is_ascii_digit() {
        let mut num = (data[i][j] - b'0') as u32;
        let mut k = j - 1;
        let mut multiplier = 10u32;
        // search left!
        while data[i][k].is_ascii_digit() {
            num = (data[i][k] - b'0') as u32 * multiplier + num;
            k -= 1;
            multiplier *= 10;
        }
        numbers.push(num);
    }
}
fn check_right(i: usize, j: usize, data: &Vec<Vec<u8>>, numbers: &mut Vec<u32>) {
    if data[i][j].is_ascii_digit() {
        let mut num = (data[i][j] - b'0') as u32;
        let mut k = j + 1;
        while data[i][k].is_ascii_digit() {
            num = num * 10 + (data[i][k] - b'0') as u32;
            k += 1;
        }
        numbers.push(num);
    }
}

fn check_adjacents_pt2(i: usize, j: usize, data: &Vec<Vec<u8>>) -> u32 {
    let mut numbers = Vec::with_capacity(4);
    // top
    check_lr(i - 1, j, data, &mut numbers);
    // bottom
    check_lr(i + 1, j, data, &mut numbers);
    // left 
    check_left(i, j - 1, data, &mut numbers);
    // right
    check_right(i, j + 1, data, &mut numbers);
    if numbers.len() == 2 { numbers[0] * numbers[1] } else { 0 }
}


pub fn part_one(input: &str) -> Option<i32> {
    let data = parse(input);
    let mut values: Vec<i32> = Vec::with_capacity(20);
    for i in 1..(data.len() - 1) { // exclude first and last row
        let mut cur_n = -1;
        let row: &[u8] = &data[i];
        let mut jstart: usize = 0;
        let mut jend: usize;
        for j in 0..row.len() {
            if row[j].is_ascii_digit() {
                let v = (row[j] - b'0') as i32;
                if cur_n > 0 {
                    cur_n = cur_n * 10 + v;
                } else {
                    cur_n = v;
                    jstart = j;
                }
            } else {
                if cur_n > 0 {
                    jend = j - 1;
                    if check_adjacents(i, jstart, jend, &data) {
                        values.push(cur_n);
                    }
                    cur_n = -1;
                }
            }
        }
    }
    Some(values.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let data = parse(input);
    let mut sum: u32 = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == b'*' {
                sum += check_adjacents_pt2(i, j, &data);
            }
        }
    }
    Some(sum)
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
