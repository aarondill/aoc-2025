use crate::utils::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Space {
    Empty,
    Paper,
}
type Input = Grid<Space>;

#[aoc_generator(day4)]
fn parse(input: &str) -> Input {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '@' => Space::Paper,
                    _ => panic!("invalid character"),
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    (0..input.width())
        .flat_map(|x| (0..input.height()).map(move |y| (x, y)))
        .filter(|&(x, y)| *input.get(x, y) == Space::Paper)
        .filter(|&(x, y)| {
            let paper_adjacent = (-1..=1)
                .into_iter()
                .flat_map(|dx| (-1..=1).into_iter().map(move |dy| (dx, dy)))
                .filter(|&(dx, dy)| dx != 0 || dy != 0)
                .map(|(dx, dy)| (x as isize + dx, y as isize + dy))
                .filter_map(|(x, y)| {
                    if x < 0 || y < 0 {
                        return None;
                    }
                    let (x, y) = (x as usize, y as usize);
                    if x < input.width() && y < input.height() { Some((x, y)) } else { None }
                })
                .filter(|&(x, y)| *input.get(x, y) == Space::Paper)
                .count();
            paper_adjacent < 4
        })
        .count()
}

// #[aoc(day4, part2)]
// fn part2(input: &Input) -> u64 {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 13);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&parse(INPUT)), 0);
    // }
}
