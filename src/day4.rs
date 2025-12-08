use grid::Grid;

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
        .collect::<Vec<_>>()
        .into()
}

#[aoc(day4, part1)]
fn part1(input: &Input) -> usize {
    input
        .indexed_iter()
        .filter(|&(_, &space)| space == Space::Paper)
        .filter(|&((y, x), _)| {
            let paper_adjacent = (-1..=1)
                .into_iter()
                .flat_map(|dy| (-1..=1).into_iter().map(move |dx| (dy, dx)))
                .filter(|&(dy, dx)| dx != 0 || dy != 0)
                .map(|(dy, dx)| (y as isize + dy, x as isize + dx))
                .filter_map(|(y, x)| {
                    if x < 0 || y < 0 {
                        return None;
                    }
                    let (y, x) = (y as usize, x as usize);
                    if x < input.cols() && y < input.rows() { Some((y, x)) } else { None }
                })
                .filter(|&pos| input[pos] == Space::Paper)
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
