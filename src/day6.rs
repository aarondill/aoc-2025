use grid::Grid;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Multiply,
}
impl Operator {
    fn from_char(c: char) -> Self {
        match c {
            '+' => Self::Add,
            '*' => Self::Multiply,
            _ => unreachable!("invalid operator"),
        }
    }

    fn apply(&self, a: u64, b: u64) -> u64 {
        match self {
            Self::Add => a + b,
            Self::Multiply => a * b,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Problem {
    values: Vec<u64>,
    operator: Operator,
}

type Input = (Vec<String>, Vec<Operator>);
#[aoc_generator(day6)]
fn parse(input: &str) -> Input {
    let lines: Vec<_> = input.lines().filter(|line| !line.is_empty()).collect();
    let (operators, lines) = lines.split_last().unwrap();
    let operators: Vec<_> =
        operators.chars().filter(|c| !c.is_whitespace()).map(Operator::from_char).collect();
    (lines.iter().map(|s| s.to_string()).collect(), operators)
}

#[aoc(day6, part1)]
fn part1(input: &Input) -> u64 {
    let (value_lines, operators) = input;
    // Row major by default
    let grid: Grid<u64> = value_lines
        .into_iter()
        .map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();
    // use the grid to transpose for us
    grid.iter_cols()
        .enumerate()
        .map(|(i, col)| Problem { values: col.copied().collect(), operator: operators[i] })
        .map(|p| p.values.into_iter().reduce(|a, b| p.operator.apply(a, b)).expect("empty values!"))
        .sum()
}

#[aoc(day6, part2)]
fn part2(input: &Input) -> u64 {
    let (value_lines, operators) = input;
    // A grid of digits, but None for empty cells
    let mut digit_grid: Grid<Option<u64>> = value_lines
        .into_iter()
        .map(|line| line.chars().map(|s| s.to_digit(10).map(|d| d as u64)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
        .into();
    // Add a row of Nones to the left for easier parsing
    digit_grid.insert_col(0, vec![None; digit_grid.rows()]);
    let digit_grid = digit_grid;

    // right to left!
    let mut iter = digit_grid.iter_cols().rev();
    operators
        .iter()
        .rev()
        .copied()
        .map(|operator| {
            let values = iter
                .by_ref()
                .map_while(|row| row.copied().filter_map(|v| v).reduce(|a, b| a * 10 + b))
                .collect::<Vec<_>>();
            assert!(!values.is_empty());
            Problem { values, operator }
        })
        .map(|p| p.values.into_iter().reduce(|a, b| p.operator.apply(a, b)).expect("empty values!"))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +
";
    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(INPUT)), 4277556);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(INPUT)), 3263827);
    }
}
