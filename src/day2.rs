#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    // todo!("Part 1")
    return input.len();
}

// #[aoc(day2, part2)]
// fn part2(input: &str) -> String {
//     todo!()
// }

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,\n1698522-1698528,446443-446449,38593856-38593862,565653-565659,\n824824821-824824827,2121212118-2121212124";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1227775554);
    }
    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(INPUT), 6);
    // }
}
