use crate::{Solution, SolutionPair};
use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::BTreeSet;

///////////////////////////////////////////////////////////////////////////////
const INPUT: &str = include_str!("../../input/day04.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
struct Card {
    id: usize,
    winners: BTreeSet<u64>,
    numbers: BTreeSet<u64>,
}

impl Card {
    fn from_str(s: &str) -> Result<Self> {
        const RE_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());

        let mut parts = s.split(|c| c == ':' || c == '|');
        let head_s = parts
            .next()
            .with_context(|| format!("Invalid input '{s}'. "))?;
        let winners_s = parts
            .next()
            .with_context(|| format!("Invalid input '{s}'. "))?;
        let numbers_s = parts
            .next()
            .with_context(|| format!("Invalid input '{s}'. "))?;

        let id: usize = RE_NUMBER
            .find(head_s)
            .with_context(|| format!("Invalid input '{s}'."))?
            .as_str()
            .parse()?;

        let mut winners = BTreeSet::<u64>::new();
        for num_match in RE_NUMBER.find_iter(winners_s) {
            let num_s = num_match.as_str();
            let num: u64 = num_s
                .parse()
                .with_context(|| format!("Invalid number '{num_s}'."))?;
            winners.insert(num);
        }

        let mut numbers = BTreeSet::<u64>::new();
        for num_match in RE_NUMBER.find_iter(numbers_s) {
            let num_s = num_match.as_str();
            let num: u64 = num_s
                .parse()
                .with_context(|| format!("Invalid number '{num_s}'."))?;
            numbers.insert(num);
        }

        Ok(Self {
            id,
            winners,
            numbers,
        })
    }

    fn score(&self) -> u64 {
        2u64.pow(self.count_matching_numbers() as u32) / 2
    }

    fn count_matching_numbers(&self) -> usize {
        self.winners.intersection(&self.numbers).count()
    }
}

pub fn solve() -> Result<SolutionPair> {
    let cards = INPUT
        .lines()
        .map(|line| Card::from_str(line))
        .collect::<Result<Vec<Card>>>()?;

    let sol1: u64 = cards.iter().map(|card| card.score()).sum();

    let mut counts: Vec<_> = cards
        .iter()
        .map(|card| (card.id, 1, card.count_matching_numbers()))
        .collect();

    for i in 0..counts.len() {
        let (card_id, count, score) = counts[i];
        assert_eq!(card_id, i + 1);

        for j in 1..=score as usize {
            counts[i + j].1 += count
        }
    }

    let sol2: u64 = counts.iter().map(|(_, count, _)| count).sum();

    Ok((Solution::U64(sol1), Solution::U64(sol2)))
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    const TEST_INPUT: &str = include_str!("../../input/test/day04.txt");

    #[test]
    fn card_from_str() -> Result<()> {
        let cards = TEST_INPUT
            .lines()
            .map(|line| super::Card::from_str(line))
            .collect::<Result<Vec<super::Card>>>()?;

        let score: u64 = cards.iter().map(|card| card.score()).sum();
        assert_eq!(score, 13);
        Ok(())
    }

    #[test]
    fn part_2() -> Result<()> {
        let cards = TEST_INPUT
            .lines()
            .map(|line| super::Card::from_str(line))
            .collect::<Result<Vec<super::Card>>>()?;

        let mut counts: Vec<_> = cards
            .iter()
            .map(|card| (card.id, 1, card.count_matching_numbers()))
            .collect();

        for i in 0..counts.len() {
            let (card_id, count, score) = counts[i];
            assert_eq!(card_id, i + 1);

            for j in 1..=score as usize {
                match counts.get_mut(i + j) {
                    Some((_, new_count, _)) => *new_count += count,
                    None => continue,
                }
            }
        }
        let total: u64 = counts.iter().map(|(_, count, _)| count).sum();
        assert_eq!(total, 30);
        Ok(())
    }
}
