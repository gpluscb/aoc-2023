use itertools::Itertools;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Game {
    id: u64,
    cube_collections: Vec<CubeCollection>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct CubeCollection {
    red: u64,
    green: u64,
    blue: u64,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum ParseError {
    InvalidGameDef,
    InvalidGameId,
    UnrecognisedColor,
    DuplicateColor,
    InvalidCubeAmount,
    NoSpaceInCubeSpec,
}

impl FromStr for Game {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Examples:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red

        let (game_def, cube_collections) = s.split_once(": ").ok_or(ParseError::InvalidGameDef)?;

        if !game_def.starts_with("Game ") {
            return Err(ParseError::InvalidGameDef);
        }

        let game_id: u64 = game_def["Game ".len()..game_def.len()]
            .parse()
            .map_err(|_| ParseError::InvalidGameId)?;

        let cube_collections = cube_collections
            .split("; ")
            .map(CubeCollection::from_str)
            .try_collect()?;

        Ok(Game {
            id: game_id,
            cube_collections,
        })
    }
}

impl FromStr for CubeCollection {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Examples:
        // "8 green, 6 blue, 20 red"
        // "1 blue, 2 green"
        let mut red = None;
        let mut green = None;
        let mut blue = None;

        s.split(", ").try_for_each(|part| {
            let (amount, color) = part.split_once(' ').ok_or(ParseError::NoSpaceInCubeSpec)?;

            let amount: u64 = amount.parse().map_err(|_| ParseError::InvalidCubeAmount)?;

            match color {
                "red" => {
                    if red.replace(amount).is_some() {
                        return Err(ParseError::DuplicateColor);
                    }
                }
                "green" => {
                    if green.replace(amount).is_some() {
                        return Err(ParseError::DuplicateColor);
                    }
                }
                "blue" => {
                    if blue.replace(amount).is_some() {
                        return Err(ParseError::DuplicateColor);
                    }
                }
                _ => return Err(ParseError::UnrecognisedColor),
            };

            Ok(())
        })?;

        Ok(CubeCollection {
            red: red.unwrap_or(0),
            green: green.unwrap_or(0),
            blue: blue.unwrap_or(0),
        })
    }
}

impl CubeCollection {
    fn is_part_1_possible(&self) -> bool {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
}

pub fn run_part_1(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Game>().expect("Parse error"))
        .filter(|game| {
            game.cube_collections
                .iter()
                .all(CubeCollection::is_part_1_possible)
        })
        .map(|game| game.id)
        .sum::<u64>()
        .to_string()
}

pub fn run_part_2(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<Game>().expect("Parse error"))
        .map(|game| {
            let min_red = game
                .cube_collections
                .iter()
                .map(|collection| collection.red)
                .max()
                .unwrap_or(0);

            let min_green = game
                .cube_collections
                .iter()
                .map(|collection| collection.green)
                .max()
                .unwrap_or(0);

            let min_blue = game
                .cube_collections
                .iter()
                .map(|collection| collection.blue)
                .max()
                .unwrap_or(0);

            min_red * min_green * min_blue
        })
        .sum::<u64>()
        .to_string()
}
