use anyhow::{anyhow, Result};

const NUMERICS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

/// Trait for turning string types into numeric digits
trait StringDigit {
    fn to_u64(self) -> Result<u64>;
}

impl StringDigit for &str {
    fn to_u64(self) -> Result<u64> {
        let result = match self {
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.parse()?,
            "zero" => 0,
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            _ => return Err(anyhow!("not a valid digit")),
        };

        Ok(result)
    }
}

fn extract_first_and_last_digits(text: &str) -> Result<u64> {
    let digits: Vec<char> = text.chars().filter(|x| x.is_numeric()).collect();
    let value = match (digits.first(), digits.last()) {
        (Some(first), Some(last)) => format!("{first}{last}").parse()?,
        _ => return Err(anyhow!("no digits in string")),
    };
    Ok(value)
}

fn extract_first_and_last_digit_or_numeric_word(text: &str) -> Result<u64> {
    let digits = filter_digits_and_numeric_words(text)?;
    let value = match (digits.first(), digits.last()) {
        (Some(first), Some(last)) => first * 10 + last,
        _ => return Err(anyhow!("no digits in string")),
    };
    Ok(value)
}

fn filter_digits_and_numeric_words(text: &str) -> Result<Vec<u64>> {
    let mut digits: Vec<(usize, &str)> = vec![];
    for digit in NUMERICS {
        let mut matches: Vec<(usize, &str)> = text.match_indices(digit).collect();
        digits.append(&mut matches)
    }
    digits.sort_by_key(|x| x.0);
    digits.into_iter().map(|x| x.1.to_u64()).collect()
}

///
/// Part one of the puzzle involves scanning each line, creating a two
/// digit number using the first and last numeric characters found in
/// each line, then summing the two digit numbers from all of the lines.
///
/// ```
/// use day1::solve_part_one;
/// use std::fs;
///
/// let text = fs::read_to_string("src/part1_example.txt").unwrap();
/// let result = solve_part_one(&text).unwrap();
/// assert_eq!(result, 142)
/// ```
///
pub fn solve_part_one(text: &str) -> Result<u64> {
    // we'll solve this using a procedural approach since it's both fast
    // and easy to read.
    let mut total = 0;
    for line in text.lines() {
        total += extract_first_and_last_digits(line)?;
    }
    Ok(total)
}

///
/// Part two of the puzzle modifies the requirements from part one.
/// We are still finding the first and last numbers, but this time
/// words that represent numeric values *also* count as valid digits.
///
/// ```
/// use day1::solve_part_two;
/// use std::fs;
///
/// let text = fs::read_to_string("src/part2_example.txt").unwrap();
/// let result = solve_part_two(&text).unwrap();
/// assert_eq!(result, 281)
/// ```
///
pub fn solve_part_two(text: &str) -> Result<u64> {
    // we'll solve this using a procedural approach since it's both fast
    // and easy to read.
    let mut total = 0;
    for line in text.lines() {
        total += extract_first_and_last_digit_or_numeric_word(line)?;
    }
    Ok(total)
}

pub mod mt {
    use super::*;
    use rayon::prelude::*;

    pub fn solve_part_one(text: &str) -> Result<u64> {
        let nums: Vec<u64> = text
            .par_lines()
            .map(extract_first_and_last_digits)
            .collect::<Result<Vec<u64>>>()?;
        let total: u64 = nums.par_iter().sum();
        Ok(total)
    }

    pub fn solve_part_two(text: &str) -> Result<u64> {
        let nums: Vec<u64> = text
            .par_lines()
            .map(extract_first_and_last_digit_or_numeric_word)
            .collect::<Result<Vec<u64>>>()?;
        let total: u64 = nums.par_iter().sum();
        Ok(total)
    }
}

pub fn print_answers(text: &str) -> Result<()> {
    let part_one = solve_part_one(text)?;
    let part_two = solve_part_two(text)?;

    println!("part one: {part_one}");
    println!("part two: {part_two}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn begins_and_ends_with_number() -> Result<()> {
        let text = "1abc2";
        let result = extract_first_and_last_digits(text)?;
        Ok(assert_eq!(result, 12))
    }

    #[test]
    fn begins_and_ends_with_letter() -> Result<()> {
        let text = "pqr3stu8vwx";
        let result = extract_first_and_last_digits(text)?;
        Ok(assert_eq!(result, 38))
    }

    #[test]
    fn has_multiple_numbers() -> Result<()> {
        let text = "a1b2c3d4e5f";
        let result = extract_first_and_last_digits(text)?;
        Ok(assert_eq!(result, 15))
    }

    #[test]
    fn has_one_number() -> Result<()> {
        let text = "treb7uchet";
        let result = extract_first_and_last_digits(text)?;
        Ok(assert_eq!(result, 77))
    }
}
