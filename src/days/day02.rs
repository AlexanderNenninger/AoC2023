use crate::{Solution, SolutionPair};
use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use pcre2::bytes::Regex;
use std::str;
///////////////////////////////////////////////////////////////////////////////
const INPUT: &str = include_str!("../../input/day02.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
struct Game {
    id: u64,
    rounds: Vec<Round>,
}

impl Game {
    const RE_ID: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?<=Game\s)\d+(?=:)").unwrap());
    const RE_ROUND: Lazy<Regex> = Lazy::new(|| Regex::new(r"([a-z\d\s,]+)(;|$)").unwrap());

    fn from_bytes(s: &[u8]) -> Result<Self> {
        let id_b = Self::RE_ID.find(s).unwrap().unwrap().as_bytes();
        let id: u64 = str::from_utf8(id_b)?.parse()?;

        let mut rounds = vec![];
        for cap in Self::RE_ROUND.captures_iter(s) {
            let cap = cap?;
            let round_b = cap
                .get(1)
                .with_context(|| format!("Round not found in {cap:?}"))?
                .as_bytes();
            rounds.push(Round::from_bytes(round_b)?)
        }

        Ok(Game { id, rounds })
    }

    fn is_possible(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
        self.rounds
            .iter()
            .all(|round| round.is_possible(max_red, max_green, max_blue))
    }

    fn min_set(&self) -> Round {
        let init = Round::new(0, 0, 0);
        self.rounds.iter().fold(init, |r, s| r.max(s))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct Round {
    num_red: u64,
    num_green: u64,
    num_blue: u64,
}

impl Round {
    fn new(num_red: u64, num_green: u64, num_blue: u64) -> Self {
        Round {
            num_red,
            num_green,
            num_blue,
        }
    }

    const RE_COLORS: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"(\d+)(?:\s(red|green|blue))").unwrap());

    fn from_bytes(s: &[u8]) -> Result<Self> {
        let mut round = Round::new(0, 0, 0);

        for cap in Self::RE_COLORS.captures_iter(s).take(3) {
            let cap = cap?;
            let count_b = cap
                .get(1)
                .with_context(|| format!("Count not found in {cap:?}"))?
                .as_bytes();
            let color_b = cap
                .get(2)
                .with_context(|| format!("Color not found in {cap:?}"))?
                .as_bytes();

            let count = str::from_utf8(count_b)?;
            let color = str::from_utf8(color_b)?;

            match color {
                "red" => round.num_red = count.parse()?,
                "green" => round.num_green = count.parse()?,
                "blue" => round.num_blue = count.parse()?,
                _ => return Err(anyhow!("Unknown color {color}.")),
            };
        }
        Ok(round)
    }

    fn is_possible(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
        self.num_red <= max_red && self.num_green <= max_green && self.num_blue <= max_blue
    }

    fn max(&self, other: &Self) -> Self {
        Round::new(
            self.num_red.max(other.num_red),
            self.num_green.max(other.num_green),
            self.num_blue.max(other.num_blue),
        )
    }

    fn power(&self) -> u64 {
        self.num_blue * self.num_green * self.num_red
    }
}

fn part_1(games: &[Game]) -> u64 {
    const MAX_RED: u64 = 12;
    const MAX_GREEN: u64 = 13;
    const MAX_BLUE: u64 = 14;

    games
        .iter()
        .filter(|game| game.is_possible(MAX_RED, MAX_GREEN, MAX_BLUE))
        .map(|game| game.id)
        .sum::<u64>()
}

fn part_2(games: &[Game]) -> u64 {
    games.iter().map(|game| game.min_set().power()).sum()
}

pub fn solve() -> Result<SolutionPair> {
    let games: Vec<Game> = INPUT
        .lines()
        .map(|line| Game::from_bytes(line.as_bytes()))
        .collect::<Result<Vec<Game>>>()?;

    let sol1: u64 = part_1(&games);
    let sol2: u64 = part_2(&games);

    Ok((Solution::U64(sol1), Solution::U64(sol2)))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::days::day02::Round;

    #[test]
    fn round_from_str() -> Result<()> {
        const TEST_STR: &str = "6 red, 1 blue, 3 green";
        let round = super::Round::from_bytes(TEST_STR.as_bytes())?;
        assert_eq!(round, Round::new(6, 3, 1));
        Ok(())
    }

    #[test]
    fn round_max() -> Result<()> {
        const ROUND_1_STR: &str = "6 red, 1 blue, 3 green";
        const ROUND_2_STR: &str = "6 red, 2 blue, 1 green";
        let round_1 = super::Round::from_bytes(ROUND_1_STR.as_bytes())?;
        let round_2 = super::Round::from_bytes(ROUND_2_STR.as_bytes())?;

        let round = round_1.max(&round_2);
        assert_eq!(round, Round::new(6, 3, 2));
        Ok(())
    }

    #[test]
    fn game_from_str() -> Result<()> {
        const TEST_STR: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = super::Game::from_bytes(TEST_STR.as_bytes())?;
        assert_eq!(game.min_set().power(), 48);

        assert_eq!(
            game,
            super::Game {
                id: 1,
                rounds: vec![
                    Round {
                        num_red: 4,
                        num_green: 0,
                        num_blue: 3,
                    },
                    Round {
                        num_red: 1,
                        num_green: 2,
                        num_blue: 6,
                    },
                    Round {
                        num_red: 0,
                        num_green: 2,
                        num_blue: 0,
                    },
                ],
            }
        );

        Ok(())
    }
}
