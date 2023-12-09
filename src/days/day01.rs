use crate::{Solution, SolutionPair};
use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;

const INPUT: &str = include_str!("../../input/day01.txt");
static RE_DIGIT: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d").unwrap());
static RE_ANY_NUMBER: Lazy<Regex> =
    Lazy::new(|| Regex::new("(\\d|one|two|three|four|five|six|seven|eight|nine)").unwrap());
///////////////////////////////////////////////////////////////////////////////

fn parse_input<'a>(data: &'a str, re: &'a Regex) -> Result<Vec<(u64, u64)>> {
    data.lines()
        .map(|line| {
            // We need to search twice, since first and last digit could be the same.
            let first_digit = re
                .find_iter(line)
                .next()
                .with_context(|| format!("No digit in line '{line}'"))?
                .as_str();

            let last_digit: &str = re
                .find_iter(line)
                .last()
                .with_context(|| format!("No second digit in '{line}'"))?
                .as_str();

            Ok((read_digit(first_digit)?, read_digit(last_digit)?))
        })
        .collect::<Result<Vec<(u64, u64)>>>()
}

fn calculate_result(data: &str, re: &Regex) -> Result<u64> {
    let mut res = 0;
    for (d1, d2) in parse_input(data, re)? {
        res += 10 * d1 + d2
    }
    Ok(res)
}

fn read_digit(d: &str) -> Result<u64> {
    match d {
        "1" | "one" => Ok(1),
        "2" | "two" => Ok(2),
        "3" | "three" => Ok(3),
        "4" | "four" => Ok(4),
        "5" | "five" => Ok(5),
        "6" | "six" => Ok(6),
        "7" | "seven" => Ok(7),
        "8" | "eight" => Ok(8),
        "9" | "nine" => Ok(9),
        _ => Err(anyhow!("Not a valid digit.")),
    }
}

pub fn solve() -> Result<SolutionPair> {
    let sol1: u64 = calculate_result(INPUT, &RE_DIGIT)?;
    let sol2: u64 = calculate_result(INPUT, &RE_ANY_NUMBER)?;

    Ok((Solution::U64(sol1), Solution::U64(sol2)))
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = include_str!("../../input/test/day01.txt");

    #[test]
    fn parse_input() -> super::Result<()> {
        let _ = super::parse_input(TEST_INPUT, &super::RE_ANY_NUMBER)?;
        Ok(())
    }

    #[test]
    fn calculate_result() -> super::Result<()> {
        let result = super::calculate_result(TEST_INPUT, &super::RE_ANY_NUMBER)?;
        assert_eq!(result, 281);
        Ok(())
    }
}
