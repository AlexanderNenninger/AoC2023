use crate::{Solution, SolutionPair};
use anyhow::{anyhow, Context, Result};
use once_cell::sync::Lazy;
use regex::Regex;
use std::str;

///////////////////////////////////////////////////////////////////////////////
const INPUT: &str = include_str!("../../input/day03.txt");

//

#[derive(Debug, Clone, PartialEq, Eq)]
struct Schema {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
struct Coordinate {
    row: usize,
    col: usize,
    nrows: usize,
    ncols: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Number {
    value: usize,
    coordinates: Vec<Coordinate>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Symbol {
    value: char,
    coordinate: Coordinate,
}

impl Schema {
    fn from_str(s: &str) -> Result<Self> {
        const RE_NUMBER: Lazy<Regex> = Lazy::new(|| Regex::new(r"\d+").unwrap());
        const RE_SYMBOL: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"(\+|/|\-|\$|=|&|#|%|@|\*)").unwrap());

        let nrows = s.lines().count();
        let ncols = s
            .lines()
            .next()
            .with_context(|| "Empty String.")?
            .chars()
            .count();

        let numbers: Vec<_> = RE_NUMBER
            .find_iter(s)
            .map(|re_match| {
                let coordinates: Vec<_> = (re_match.start()..re_match.end())
                    .map(|k| Coordinate::from_linear(k, nrows, ncols + 1))
                    .collect();

                let value: usize = re_match.as_str().parse().expect("Regex Error");
                Number { value, coordinates }
            })
            .collect();

        let symbols: Vec<Symbol> = RE_SYMBOL
            .find_iter(s)
            .map(|re_match| {
                let coordinate = Coordinate::from_linear(re_match.start(), nrows, ncols + 1);
                let value = re_match.as_str();
                Symbol::from_str(value, coordinate)
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Self { numbers, symbols })
    }
}

impl Coordinate {
    fn from_linear(k: usize, nrows: usize, ncols: usize) -> Coordinate {
        Coordinate {
            row: k / ncols,
            col: k % ncols,
            nrows,
            ncols,
        }
    }

    fn is_adjacent(&self, other: &Self) -> bool {
        (self.row as isize - other.row as isize)
            .abs()
            .max((self.col as isize - other.col as isize).abs())
            <= 1
    }
}

impl Symbol {
    const SYMBOLS: [char; 10] = ['+', '/', '-', '$', '=', '&', '#', '%', '@', '*'];
    fn from_str(s: &str, coordinate: Coordinate) -> Result<Self> {
        if s.len() != 1 {
            return Err(anyhow!(
                "Could not read symbol. String needs to be of length 1."
            ));
        }
        match Self::SYMBOLS
            .into_iter()
            .find(|&c| s.chars().next().expect("Did not catch s.len() == 0") == c)
        {
            Some(value) => Ok(Symbol { value, coordinate }),
            None => Err(anyhow!("Not a valid symbol")),
        }
    }

    fn is_adjacent(&self, other: &Number) -> bool {
        other.is_adjacent(&self.coordinate)
    }

    fn gear_ratio(&self, numbers: &[Number]) -> Option<usize> {
        let candidates: Vec<_> = numbers
            .iter()
            .filter(|&number| self.is_adjacent(number))
            .collect();

        if candidates.len() != 2 {
            return None;
        }

        Some(candidates[0].value * candidates[1].value)
    }
}

impl Number {
    fn is_adjacent(&self, other: &Coordinate) -> bool {
        self.coordinates
            .iter()
            .any(|coord| coord.is_adjacent(other))
    }
}

fn part_1(schema: &Schema) -> u64 {
    schema
        .numbers
        .iter()
        .filter(|num| {
            schema
                .symbols
                .iter()
                .any(|symbol| num.is_adjacent(&symbol.coordinate))
        })
        .map(|num| num.value)
        .sum::<usize>() as u64
}

fn part_2(schema: &Schema) -> u64 {
    schema
        .symbols
        .iter()
        .filter_map(|symbol| symbol.gear_ratio(&schema.numbers))
        .sum::<usize>() as u64
}

pub fn solve() -> Result<SolutionPair> {
    let schema = Schema::from_str(INPUT)?;
    let sol1: u64 = part_1(&schema);
    let sol2: u64 = part_2(&schema);

    Ok((Solution::U64(sol1), Solution::U64(sol2)))
}

#[cfg(test)]
mod tests {

    const TEST_DATA_01: &str = include_str!("../../input/test/day03_01.txt");

    #[test]
    fn is_adjacent() {
        let schema = super::Schema::from_str(TEST_DATA_01).unwrap();
        let n1 = schema.numbers[1].clone();
        let s1 = schema.symbols[0].clone();
        let s2 = schema.symbols[1].clone();

        dbg!(&n1);
        dbg!(&s1);
        dbg!(&s2);

        assert_eq!(n1.is_adjacent(&s1.coordinate), false);
        assert_eq!(n1.is_adjacent(&s2.coordinate), false);
    }

    #[test]
    fn part_1() {
        let schema = super::Schema::from_str(TEST_DATA_01).unwrap();
        let part_numbers: usize = schema
            .numbers
            .iter()
            .filter(|num| {
                schema
                    .symbols
                    .iter()
                    .any(|symbol| num.is_adjacent(&symbol.coordinate))
            })
            .map(|num| num.value)
            .sum();
        assert_eq!(part_numbers, 4361);
    }

    #[test]
    fn part_2() {
        let schema = super::Schema::from_str(TEST_DATA_01).unwrap();
        assert_eq!(super::part_2(&schema), 467835)
    }
}
