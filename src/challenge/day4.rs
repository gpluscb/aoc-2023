use itertools::Itertools;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ParseError {
    InvalidScratchCardDef,
    InvalidNumbersSection,
    InvalidOwnNumber,
    InvalidWinningNumber,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct ScratchCard {
    own_numbers: Vec<u64>,
    winning_numbers: Vec<u64>,
}

impl ScratchCard {
    fn points(&self) -> u64 {
        let matching_count = self.matching_count();

        if matching_count == 0 {
            0
        } else {
            2u64.pow(matching_count as u32 - 1)
        }
    }

    fn matching_count(&self) -> usize {
        self.own_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .count()
    }
}

impl FromStr for ScratchCard {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Examples:
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1

        let (card_def, all_numbers) = s
            .split_once(": ")
            .ok_or(ParseError::InvalidScratchCardDef)?;
        let (own_numbers, winning_numbers) = all_numbers
            .split_once(" | ")
            .ok_or(ParseError::InvalidNumbersSection)?;

        if !card_def.starts_with("Card ") {
            return Err(ParseError::InvalidScratchCardDef);
        }

        let own_numbers = own_numbers
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u64>().map_err(|_| ParseError::InvalidOwnNumber))
            .try_collect()?;

        let winning_numbers = winning_numbers
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.parse::<u64>()
                    .map_err(|_| ParseError::InvalidWinningNumber)
            })
            .try_collect()?;

        Ok(ScratchCard {
            own_numbers,
            winning_numbers,
        })
    }
}

pub fn run_part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<ScratchCard>().expect("Parse error"))
        .map(|scratch_card| scratch_card.points())
        .sum::<u64>()
        .to_string()
}

pub fn run_part_2(input: &str) -> String {
    let mut scratch_card_points_and_copies: Vec<(usize, u64)> = input
        .lines()
        .map(|line| line.parse::<ScratchCard>().expect("Parse error"))
        .map(|scratch_card| (scratch_card.matching_count(), 1))
        .collect();

    for i in 0..scratch_card_points_and_copies.len() {
        let (points, copies) = scratch_card_points_and_copies[i];

        for j in 1..(points + 1) {
            scratch_card_points_and_copies[i + j].1 += copies;
        }
    }

    scratch_card_points_and_copies
        .into_iter()
        .map(|(_points, copies)| copies)
        .sum::<u64>()
        .to_string()
}
