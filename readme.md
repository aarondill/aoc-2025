# Advent of Code 2025

Implemented using [`cargo-aoc`](https://github.com/gobanos/cargo-aoc) for simplicity.

## Installation

1. `cargo install cargo-aoc`
2. `cargo aoc credentials {token}` (See the `cargo-aoc` [README](https://github.com/gobanos/cargo-aoc#setting-up-the-cli) for more info)
3. `cargo aoc input -a` to download all inputs, or `cargo aoc input -y 2025 -d {day}` to download a specific day's input
4. `cargo aoc -d {day} -p {part}` to run a solution

## Development

1. Install above steps 1-3
2. Create a file in `src/day{day}.rs`
3. Add `pub mod day{day};` to `src/lib.rs`
4. Mark solution functions with `#[aoc(day{day}, part{part})]`
5. Add a test module to `src/day{day}.rs` to test against the sample input and output

```rust
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "{sample input}";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), expected_output_1);
    }
}
```

6. Run `cargo test day{day}` to test your solution
7. Run `cargo aoc -d {day} -p {part}` to test your solution against the real input
   - `cargo aoc` automatically runs the most recently modified solution
