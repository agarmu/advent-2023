advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<usize> {
    let l1 = input.lines().next().unwrap_or("");
    Some(l1.split(",").map(|x| hash(x.trim().as_bytes()) as usize).sum())
}

type Map<'a> = [Vec<(String, i32)>; 256];
pub fn part_two(input: &str) -> Option<i32> {
    let mut map: Map = std::array::from_fn(|_| Vec::with_capacity(10));
    let items =  input.lines().next().unwrap_or("").split(",").map(|x| x.trim());
    for item in items {
        handle_token(item, &mut map);
    }
    let mut power = 0;
    for (i, slot) in map.iter().enumerate() {
        for (j, (_, val)) in slot.iter().enumerate() {
            power += val * (i as i32 + 1) * (j as i32 + 1);
        }
    }
    Some(power)
}

pub fn handle_token<'a, 'b>(token: &'a str, map: &'b mut Map)
{
    if token.ends_with("-") {
        //removal
        let t = &token.as_bytes()[0..(token.len() - 1)];
        let hash = hash(t);
        let row = &mut map[hash as usize];
        row.retain(|x| x.0.as_bytes() != t);
    } else {
        let (t, v) = token.split_once("=").expect("if not -, then =");
        let hash = hash(t.as_bytes());
        let new_val = v.parse::<i32>().expect("hehe");
        let row = &mut map[hash as usize];
        let q = row.iter().enumerate().filter_map(|x| if x.1.0 == t { Some(x.0) } else { None }).next();
        let Some(idx) = q else {
            row.push((t.to_owned(), new_val));
            return;
        };
        row[idx] = (t.to_owned(), new_val);
    }
}

pub fn hash(x: &[u8]) -> u8 {
    let mut hash = 0u8;
    for &b in x.into_iter() {
        hash = hash.wrapping_add(b).wrapping_mul(17);
    }
    hash
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
