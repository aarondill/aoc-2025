type Rotation = i32;
type Input = Vec<Rotation>;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Input {
    input
        .lines()
        // Yeah, yeah, mutation in a filter is bad, but it works.
        .map(|line| {
            let mut iter = line.chars();
            let dir = match iter.next() {
                Some('L') => -1,
                Some('R') => 1,
                Some(_) => panic!("Invalid input: invalid direction"),
                None => panic!("Invalid input: no direction"),
            };
            let mag: i32 = iter.collect::<String>().parse().expect("Invalid input: not a number");
            mag * dir
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Input) -> usize {
    let mut dial = 50;
    input
        .iter()
        // Yeah, yeah, mutation in a filter is bad, but it works.
        .filter(|&&rotation| {
            let change = rotation % 100;
            dial = (dial + change) % 100;
            if dial < 0 {
                dial += 100;
            }
            dial == 0
        })
        .count()
}
#[aoc(day1, part2)]
pub fn part2(input: &Input) -> usize {
    let mut dial = 50;
    let mut count = 0;
    for rot in input {
        // Rust is fast enough that this actually works, despite the fact that this is a very slow method.
        for _ in 0..rot.abs() {
            dial = (dial + rot.signum()) % 100;
            if dial < 0 {
                dial += 100;
            }
            if dial == 0 {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 3);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 6);
    }
}
