use std::convert::Infallible;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct SymbolPosition {
    line: usize,
    col: usize,
    is_star: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct NumberPosition {
    number: u64,
    line: usize,
    start_col: usize,
    /// Inclusive
    end_col: usize,
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct SchematicInfo {
    symbols: Vec<SymbolPosition>,
    numbers: Vec<NumberPosition>,
}

impl FromStr for SchematicInfo {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols = Vec::new();
        let mut numbers = Vec::new();

        for (line_number, line) in s.lines().enumerate() {
            let mut curr_number_start_col = None;
            let line_width = line.len();

            for (col, character) in line.bytes().enumerate() {
                if !character.is_ascii() {
                    continue;
                }

                let is_digit = character.is_ascii_digit();

                if is_digit && curr_number_start_col.is_none() {
                    curr_number_start_col = Some(col);
                } else if let Some(number_start_col) = curr_number_start_col {
                    if !is_digit || col + 1 == line_width {
                        let last_digit_col = if is_digit { line_width - 1 } else { col - 1 };

                        let slice = &line[number_start_col..=last_digit_col];

                        let number: u64 = slice.parse().unwrap();

                        numbers.push(NumberPosition {
                            number,
                            line: line_number,
                            start_col: number_start_col,
                            end_col: last_digit_col,
                        });

                        curr_number_start_col = None;
                    }
                }

                if is_symbol(character) {
                    symbols.push(SymbolPosition {
                        line: line_number,
                        col,
                        is_star: character == b'*',
                    })
                }
            }
        }

        Ok(SchematicInfo { symbols, numbers })
    }
}

impl SchematicInfo {
    fn get_numbers_adjacent_to_symbol(&self) -> impl Iterator<Item = u64> + '_ {
        self.numbers
            .iter()
            .filter(|number_position| {
                let line_range =
                    number_position.line.saturating_sub(1)..=number_position.line.saturating_add(1);
                let col_range = number_position.start_col.saturating_sub(1)
                    ..=number_position.end_col.saturating_add(1);

                self.symbols.iter().any(|symbol_position| {
                    line_range.contains(&symbol_position.line)
                        && col_range.contains(&symbol_position.col)
                })
            })
            .map(|number_position| number_position.number)
    }

    fn get_gear_ratios(&self) -> impl Iterator<Item = u64> + '_ {
        self.symbols.iter().filter_map(|symbol_position| {
            let line_range =
                symbol_position.line.saturating_sub(1)..=symbol_position.line.saturating_add(1);
            let col_range =
                symbol_position.col.saturating_sub(1)..=symbol_position.col.saturating_add(1);

            let (total_adjacent_numbers, product): (usize, u64) = self
                .numbers
                .iter()
                .filter(|number_position| {
                    line_range.contains(&number_position.line)
                        && overlaps(
                            &col_range,
                            &(number_position.start_col..=number_position.end_col),
                        )
                })
                .fold((0, 1), |(acc_amount, acc_product), next| {
                    (acc_amount + 1, acc_product * next.number)
                });

            (total_adjacent_numbers == 2).then_some(product)
        })
    }
}

fn is_symbol(ascii: u8) -> bool {
    ascii.is_ascii() && ascii != b'.' && !ascii.is_ascii_control() && !ascii.is_ascii_alphanumeric()
}

fn overlaps(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start())
}

pub fn run_part_1(input: &str) -> String {
    input
        .parse::<SchematicInfo>()
        .unwrap()
        .get_numbers_adjacent_to_symbol()
        .sum::<u64>()
        .to_string()
}

pub fn run_part_2(input: &str) -> String {
    input
        .parse::<SchematicInfo>()
        .unwrap()
        .get_gear_ratios()
        .sum::<u64>()
        .to_string()
}
