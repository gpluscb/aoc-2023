use chrono::{DateTime, SecondsFormat, Utc};
use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;
use strum::{EnumIter, IntoEnumIterator};

mod day;

#[derive(Clone, Eq, PartialEq, Debug)]
struct TestFailure {
    day: Day,
    input: String,
    expected_output: String,
    actual_output: String,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, EnumIter)]
#[repr(u8)]
enum Day {
    Day1 = 1,
}

impl Day {
    fn from_number(day_number: u8) -> Option<Day> {
        Day::iter().find(|&day| day as u8 == day_number)
    }

    fn run(&self, part: u8, input: &str) -> String {
        match (self, part) {
            (Day::Day1, 1) => day::day1::run_part_1(input),
            (Day::Day1, 2) => day::day1::run_part_2(input),
            _ => panic!("No matching challenge"),
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

    let day = Day::from_number(current_day_number).expect("CURRENT_DAY is not a valid day");

    let current_part_number: u8 = std::env::var("CURRENT_PART")
        .expect("Provide CURRENT_PART environment variable")
        .parse()
        .expect("CURRENT_PART environment variable must be u8");

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

    let output = day.run(current_part_number, &input);

    fs::write(out_file_path, output).expect("Could not write");
}

#[cfg(test)]
mod test {
    use crate::{Day, TestFailure};
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

        for day in Day::iter() {
            let day_number = day as u8;

            let day_dir_path = base_path.join(format!("day{day_number}"));

            for part_dir in day_dir_path.read_dir().expect("Missing day test directory") {
                let part_dir_path = part_dir
                    .expect("Problem reading files in day test directory")
                    .path();

                let part_dir_name = part_dir_path
                    .file_name()
                    .expect("Weird entry in day test directory")
                    .to_str()
                    .expect("Part dir was not unicode");

                let part_number: u8 = part_dir_name["part".len()..]
                    .parse()
                    .expect("Part dir did not follow format");

                for day_test_dir in part_dir_path
                    .read_dir()
                    .expect("Missing day part directory")
                {
                    let day_test_dir_path = day_test_dir
                        .expect("Problem reading files in day part directory")
                        .path();

                    let in_file_path = day_test_dir_path.join("in.txt");
                    let out_file_path = day_test_dir_path.join("out.txt");

                    let input = fs::read_to_string(in_file_path).expect("Could not read in file");
                    let expected_output =
                        fs::read_to_string(out_file_path).expect("Could not read out file");

                    let actual_output = day.run(part_number, &input);

                    if actual_output != expected_output {
                        let failure = TestFailure {
                            day,
                            input,
                            expected_output,
                            actual_output,
                        };

                        failures.push(failure);
                    }
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
