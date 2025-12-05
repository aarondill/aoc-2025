#[aoc(day1, part1)]
pub fn part1(input: &str) -> usize {
    let mut dial: i32 = 50;
    input
        .lines()
        // Yeah, yeah, mutation in a filter is bad, but it works.
        .filter(|line| {
            let dir = line.chars().next().expect("Invalid input: no direction");
            let dir: i8 = match dir {
                'L' => -1,
                'R' => 1,
                _ => panic!("Invalid direction"),
            };
            let mag: i32 = line
                .chars()
                .skip(1)
                .collect::<String>()
                .parse()
                .expect("Invalid input: not a number");
            let change: i32 = (mag * dir as i32) % 100;
            dial = (dial + change) % 100;
            if dial < 0 {
                dial += 100;
            }
            dial == 0
        })
        .count()
}
#[aoc(day1, part2)]
pub fn part2(input: &str) -> usize {
    let mut dial: i32 = 50;
    let mut count = 0;
    for line in input.lines() {
        let dir = line.chars().next().expect("Invalid input: no direction");
        let dir: i8 = match dir {
            'L' => -1,
            'R' => 1,
            _ => panic!("Invalid direction"),
        };
        let mag: i32 =
            line.chars().skip(1).collect::<String>().parse().expect("Invalid input: not a number");
        // Rust is fast enough that this actually works, despite the fact that this is a very slow method.
        for _ in 0..mag {
            dial = (dial + dir as i32) % 100;
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
        assert_eq!(part1(INPUT), 4);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 6);
    }
}
