pub fn run_part_1(input: &str) -> String {
    input
        .lines()
        .map(extract_first_last_ascii_digits)
        // Sum may not fit into u8, u64 is gonna be reasonable hopefully :)
        .map(|(digit1, digit2)| (10 * digit1 + digit2) as u64)
        .sum::<u64>()
        .to_string()
}

fn extract_first_last_ascii_digits(input: &str) -> (u8, u8) {
    let first_digit = input
        .as_bytes()
        .iter()
        .copied()
        .find_map(extract_ascii_digit)
        .expect("No digits found");

    // Annoyingly, rfind_map doesn't exist
    let last_digit = input
        .as_bytes()
        .iter()
        .rev()
        .copied()
        .find_map(extract_ascii_digit)
        .unwrap();

    (first_digit, last_digit)
}

fn extract_ascii_digit(ascii: u8) -> Option<u8> {
    // Digits '0', '1', etc. are directly next to each other in ascii/UTF-8
    ascii.is_ascii_digit().then_some(ascii - b'0')
}

pub fn run_part_2(input: &str) -> String {
    input
        .lines()
        .map(extract_first_last_ascii_or_spelled_digits)
        // Sum may not fit into u8, u64 is gonna be reasonable hopefully :)
        .map(|(digit1, digit2)| (10 * digit1 + digit2) as u64)
        .sum::<u64>()
        .to_string()
}

fn extract_first_last_ascii_or_spelled_digits(input: &str) -> (u8, u8) {
    let first_digit = extract_first_ascii_or_spelled_digit(input).expect("No digits found");
    let last_digit = extract_last_ascii_or_spelled_digit(input).unwrap();

    (first_digit, last_digit)
}

fn extract_first_ascii_or_spelled_digit(input: &str) -> Option<u8> {
    let spelled_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (char_idx, current_char) in input.char_indices() {
        if !current_char.is_ascii() {
            continue;
        }

        let current_char = current_char as u8;

        if let Some(digit) = extract_ascii_digit(current_char) {
            return Some(digit);
        }

        for (spelled_digit_idx, &spelled) in spelled_digits.iter().enumerate() {
            // Always remember to add one to index to get the digit since we don't have zero
            let digit = spelled_digit_idx as u8 + 1;

            if input
                .get(char_idx..(char_idx + spelled.len()))
                .is_some_and(|maybe_spelled| maybe_spelled == spelled)
            {
                return Some(digit);
            }
        }
    }

    None
}

fn extract_last_ascii_or_spelled_digit(input: &str) -> Option<u8> {
    let spelled_digits = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for (char_idx, current_char) in input.char_indices().rev() {
        if !current_char.is_ascii() {
            continue;
        }

        let current_char = current_char as u8;

        if let Some(digit) = extract_ascii_digit(current_char) {
            return Some(digit);
        }

        for (spelled_digit_idx, &spelled) in spelled_digits.iter().enumerate() {
            // Always remember to add one to index to get the digit since we don't have zero
            let digit = spelled_digit_idx as u8 + 1;

            if char_idx >= spelled.len()
                && input
                    .get((char_idx + 1 - spelled.len())..=char_idx)
                    .is_some_and(|maybe_spelled| maybe_spelled == spelled)
            {
                return Some(digit);
            }
        }
    }

    None
}
