use chrono::{DateTime, SecondsFormat, Utc};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use strum::{EnumIter, IntoEnumIterator};

mod challenge;

#[derive(Clone, Eq, PartialEq, Debug)]
struct TestFailure {
    challenge: Challenge,
    input: String,
    expected_output: String,
    actual_output: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
enum Challenge {
    Day1_1,
    Day1_2,
}

impl Challenge {
    fn from_day_part(day_number: u8, part_number: u8) -> Option<Challenge> {
        Challenge::iter().find(|challenge| challenge.day_part_number() == (day_number, part_number))
    }

    fn day_part_number(&self) -> (u8, u8) {
        match self {
            Challenge::Day1_1 => (1, 1),
            Challenge::Day1_2 => (1, 2),
        }
    }

    fn run(&self, input: &str) -> String {
        match self {
            Challenge::Day1_1 => challenge::day1::run_part_1(input),
            Challenge::Day1_2 => challenge::day1::run_part_2(input),
        }
    }
}

fn main() {
    _ = dotenv::dotenv();

    let base_path = PathBuf::from(
        std::env::var("RUNDATA_DIR").expect("Provide RUNDATA_DIR environment variable"),
    );

    let current_day_number: u8 = std::env::var("CURRENT_DAY")
        .expect("Provide CURRENT_DAY environment variable")
        .parse()
        .expect("CURRENT_DAY environment variable must be u8");

    let current_part_number: u8 = std::env::var("CURRENT_PART")
        .expect("Provide CURRENT_PART environment variable")
        .parse()
        .expect("CURRENT_PART environment variable must be u8");

    let challenge = Challenge::from_day_part(current_day_number, current_part_number)
        .expect("Challenge does not exist");

    let part_run_dir_path = base_path
        .join(format!("day{current_day_number}"))
        .join(format!("part{current_part_number}"));

    let in_file_path = part_run_dir_path.join("in.txt");

    let datetime = DateTime::<Utc>::from(SystemTime::now());

    let out_file_path = part_run_dir_path.join(format!(
        "out_{}",
        datetime.to_rfc3339_opts(SecondsFormat::Secs, true)
    ));

    let input = fs::read_to_string(in_file_path).expect("Could not read in file");

    let output = challenge.run(&input);

    fs::write(out_file_path, output).expect("Could not write");
}

#[cfg(test)]
mod test {
    use crate::{Challenge, TestFailure};
    use std::fs;
    use std::path::PathBuf;
    use strum::IntoEnumIterator;

    #[test]
    fn run_tests() -> Result<(), Vec<TestFailure>> {
        _ = dotenv::dotenv();

        let mut failures = Vec::new();

        let base_path = PathBuf::from(
            std::env::var("TESTDATA_DIR").expect("Provide TESTDATA_DIR env variable"),
        );

        for challenge in Challenge::iter() {
            let (day_number, part_number) = challenge.day_part_number();

            let part_dir_path = base_path
                .join(format!("day{day_number}"))
                .join(format!("part{part_number}"));

            for day_test_dir in part_dir_path
                .read_dir()
                .expect("Missing challenge part directory")
            {
                let day_test_dir_path = day_test_dir
                    .expect("Problem reading files in challenge part directory")
                    .path();

                let in_file_path = day_test_dir_path.join("in.txt");
                let out_file_path = day_test_dir_path.join("out.txt");

                let input = fs::read_to_string(in_file_path).expect("Could not read in file");
                let expected_output =
                    fs::read_to_string(out_file_path).expect("Could not read out file");

                let actual_output = challenge.run(&input);

                if actual_output != expected_output {
                    let failure = TestFailure {
                        challenge,
                        input,
                        expected_output,
                        actual_output,
                    };

                    failures.push(failure);
                }
            }
        }

        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures)
        }
    }
}
