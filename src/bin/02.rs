advent_of_code::solution!(2);

#[derive(Debug, Clone, PartialEq)]
struct Game {
    r: u64,
    g: u64,
    b: u64,
}
impl Game {
    fn parse(x: &str) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        x.split(",").map(|x| x.trim()).for_each(|y| {
            let mut q = y.split_ascii_whitespace();
            let v = q.next().unwrap().parse::<u64>().unwrap();
            let t = q.next().unwrap();
            match t {
                "red" => r = v,
                "green" => g = v,
                "blue" => b = v,
                _ => unreachable!(),
            }
        });
        Self { r, g, b }
    }
    fn power(&self) -> u64 {
        self.r * self.g * self.b
    }
    fn is_permitted_by(&self, other: &Self) -> bool {
        self.r <= other.r && self.g <= other.g && self.b <= other.b
    }
}
impl std::ops::BitAnd<Game> for Game {
    type Output = Game;
    fn bitand(self, rhs: Game) -> Self::Output {
        Self {
            r: u64::max(self.r, rhs.r),
            g: u64::max(self.g, rhs.g),
            b: u64::max(self.b, rhs.b),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        input
            .lines()
            .map(|x| {
                let q = x.split(":").skip(1).next().unwrap().trim();
                q.split(";").map(Game::parse).reduce(|x, y| x & y).unwrap()
            })
            .enumerate()
            .filter(|(_, x)| {
                x.is_permitted_by(&Game {
                    r: 12,
                    g: 13,
                    b: 14,
                })
            })
            .map(|(i, _)| i + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .lines()
            .map(|x| {
                let q = x.split(":").skip(1).next().unwrap().trim();
                q.split(";").map(Game::parse).reduce(|x, y| x & y).unwrap()
            })
            .map(|x| x.power())
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
