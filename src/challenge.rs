use anyhow::Result;

/// A trait for solving Advent of Code challenges.
///
/// Implement this trait for a struct that represents a challenge. The struct should have a `new`
/// method that returns an instance of itself. The `part1` and `part2` methods should be
/// implemented to solve the respective parts of the challenge.
pub trait Challenge {
    /// Solve part 1 of the challenge.
    fn part1(&self, _input: &str) -> Result<String> {
        Err(anyhow::anyhow!("Part 1 not implemented"))
    }

    /// Solve part 2 of the challenge.
    fn part2(&self, _input: &str) -> Result<String> {
        Err(anyhow::anyhow!("Part 2 not implemented"))
    }
}

pub type DailyChallenge = Box<dyn Challenge>;
