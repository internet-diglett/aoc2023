use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use anyhow::Result;

#[derive(Debug)]
struct PartNumber {
    row: usize,
    begin: usize,
    end: usize,
    number: u64,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
struct SchematicSymbol {
    row: usize,
    offset: usize,
    symbol: char,
}

type LookupTable = HashMap<(usize, usize), SchematicSymbol>;

trait Symbol {
    fn is_a_symbol(&self) -> bool;
}

impl Symbol for char {
    fn is_a_symbol(&self) -> bool {
        !(self.is_ascii_digit() || *self == '.')
    }
}

enum ParserMode {
    Scanning,
    ParsingNumber,
}

/// returns a vector of possible part numbers and a hashmap of 3x3 regions mapped to their
/// symbols
fn parse(text: &str, row: usize) -> Result<(Vec<PartNumber>, LookupTable)> {
    let mut chars = text.chars().enumerate().peekable();
    let mut part_numbers: Vec<PartNumber> = vec![];
    let mut valid_positions: HashMap<(usize, usize), SchematicSymbol> = HashMap::new();
    let mut mode = ParserMode::Scanning;

    let mut current_numeric_string = String::new();
    let mut begin = 0;

    while let Some((i, c)) = chars.next() {
        match (c.is_ascii_digit(), c.is_a_symbol(), &mode) {
            // happy path

            // We are scanning and we have found the first digit of
            // a number
            (true, false, ParserMode::Scanning) => {
                mode = ParserMode::ParsingNumber;
                begin = i;
                current_numeric_string.push(c);
            }

            // We are scanning and we have found a symbol
            (false, true, ParserMode::Scanning) => {
                update_positions(row, i, c, &mut valid_positions);
            }

            // We are scanning and we have found nothing interesting
            (false, false, ParserMode::Scanning) => {
                // do nothing
            }

            // We are parsing a number and have found an additional digit
            (true, false, ParserMode::ParsingNumber) => {
                current_numeric_string.push(c);
                // finalize if we have reached the end of the line
                if chars.peek().is_none() {
                    finalize_part_number(
                        &mut mode,
                        row,
                        begin,
                        i,
                        &mut current_numeric_string,
                        &mut part_numbers,
                    )?;
                }
            }

            // We are parsing a number and have found a character that is a
            // symbol, not a number
            (false, true, ParserMode::ParsingNumber) => {
                update_positions(row, i, c, &mut valid_positions);
                finalize_part_number(
                    &mut mode,
                    row,
                    begin,
                    i - 1,
                    &mut current_numeric_string,
                    &mut part_numbers,
                )?;
            }

            // We are parsing a number and have found no additional interesting
            // characters
            (false, false, ParserMode::ParsingNumber) => {
                finalize_part_number(
                    &mut mode,
                    row,
                    begin,
                    i - 1,
                    &mut current_numeric_string,
                    &mut part_numbers,
                )?;
            }

            // sad path
            // it should not be possible for a character to be a symbol and a number
            (true, true, _) => {
                unreachable!()
            }
        }
    }
    Ok((part_numbers, valid_positions))
}

fn finalize_part_number(
    mode: &mut ParserMode,
    row: usize,
    begin: usize,
    end: usize,
    current_numeric_string: &mut String,
    part_numbers: &mut Vec<PartNumber>,
) -> Result<()> {
    *mode = ParserMode::Scanning;
    let part_number = PartNumber {
        row,
        begin,
        end,
        // this unwrap should be safe
        number: current_numeric_string.parse()?,
    };
    part_numbers.push(part_number);
    *current_numeric_string = String::new();
    Ok(())
}

fn update_positions(
    row: usize,
    i: usize,
    c: char,
    valid_positions: &mut HashMap<(usize, usize), SchematicSymbol>,
) {
    let symbol = SchematicSymbol {
        row,
        offset: i,
        symbol: c,
    };
    for y in (row.saturating_sub(1))..=(row + 1) {
        for x in (i.saturating_sub(1))..=(i + 1) {
            valid_positions.insert((x, y), symbol);
        }
    }
}

///
/// ```txt
/// The engineer explains that an engine part seems to be missing from the engine,
/// but nobody can figure out which one. If you can add up all the part numbers
/// in the engine schematic, it should be easy to work out which part is missing.
///
/// The engine schematic (your puzzle input) consists of a visual
/// representation of the engine. There are lots of numbers and symbols you
/// don't really understand, but apparently any number adjacent to a symbol,
/// even diagonally, is a "part number" and should be included in your sum.
/// (Periods (.) do not count as a symbol.)
///
/// Here is an example engine schematic:
///
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
///
/// In this schematic, two numbers are not part numbers because they are not
/// adjacent to a symbol: 114 (top right) and 58 (middle right). Every other
/// number is adjacent to a symbol and so is a part number; their sum is 4361.
/// ```
///
/// ```
/// use day3::solve_part_one;
/// use std::fs;
///
/// let text = fs::read_to_string("src/part1_example.txt").unwrap();
/// let result = solve_part_one(&text).unwrap();
/// assert_eq!(result, 4361)
/// ```
///
pub fn solve_part_one(text: &str) -> Result<u64> {
    // build a collection for the part numbers with their row number, start index,
    // and end index.
    let mut part_numbers = vec![];

    // build a lookup table for valid positions for numbers, generated by the symbols
    let mut valid_positions: HashMap<(usize, usize), SchematicSymbol> = HashMap::new();

    for (i, line) in text.lines().enumerate() {
        let (mut new_part_numbers, mut new_valid_positions) = parse(line, i)?;

        part_numbers.append(&mut new_part_numbers);

        new_valid_positions.drain().for_each(|(k, v)| {
            valid_positions.insert(k, v);
        });
    }

    // filter the collection of numbers using the lookup table for valid positions
    let valid_parts = part_numbers.iter().filter(|pn| {
        for x in pn.begin..=pn.end {
            if valid_positions.contains_key(&(x, pn.row)) {
                return true;
            }
        }
        false
    });

    // sum the numbers
    Ok(valid_parts.map(|pn| pn.number).sum())
}

///
/// ```txt
/// The missing part wasn't the only issue - one of the gears in the engine is wrong.
/// A gear is any * symbol that is adjacent to exactly two part numbers.
/// Its gear ratio is the result of multiplying those two numbers together.
/// This time, you need to find the gear ratio of every gear and add them all
/// up so that the engineer can figure out which gear needs to be replaced.
///
/// Consider the same engine schematic again:
///
/// 467..114..
/// ...*......
/// ..35..633.
/// ......#...
/// 617*......
/// .....+.58.
/// ..592.....
/// ......755.
/// ...$.*....
/// .664.598..
///
/// In this schematic, there are two gears. The first is in the top left; it has part
/// numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right;
/// its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only
/// adjacent to one part number.) Adding up all of the gear ratios produces 467835.
/// ```
///
/// ```
/// use day3::solve_part_two;
/// use std::fs;
///
/// let text = fs::read_to_string("src/part1_example.txt").unwrap();
/// let result = solve_part_two(&text).unwrap();
/// assert_eq!(result, 467835)
/// ```
///
pub fn solve_part_two(text: &str) -> Result<u64> {
    // build a collection for the part numbers with their row number, start index,
    // and end index.
    let mut part_numbers = vec![];

    // build a lookup table for valid positions for numbers, generated by the symbols
    let mut valid_positions: HashMap<(usize, usize), SchematicSymbol> = HashMap::new();

    for (i, line) in text.lines().enumerate() {
        let (mut new_part_numbers, mut new_valid_positions) = parse(line, i)?;

        part_numbers.append(&mut new_part_numbers);

        new_valid_positions.drain().for_each(|(k, v)| {
            valid_positions.insert(k, v);
        });
    }

    // build a table to store our gear ratios
    let mut unvalidated_gear_ratios: HashMap<SchematicSymbol, Vec<u64>> = HashMap::new();

    part_numbers.iter().for_each(|pn| {
        for x in pn.begin..=pn.end {
            if let Some(entry) = valid_positions.get(&(x, pn.row)) {
                if entry.symbol != '*' {
                    continue;
                }
                match unvalidated_gear_ratios.entry(*entry) {
                    Occupied(mut existing_entry) => {
                        existing_entry.get_mut().push(pn.number);
                    }
                    Vacant(new_entry) => {
                        new_entry.insert(vec![pn.number]);
                    }
                }
                break;
            }
        }
    });

    // validate our gear ratios
    let valid_gear_ratios = unvalidated_gear_ratios.iter().filter(|(_, v)| v.len() == 2);
    let sum = valid_gear_ratios
        .map(|(_, v)| v.iter().product::<u64>())
        .sum();
    Ok(sum)
}

pub fn print_answers(text: &str) -> Result<()> {
    let part_one = solve_part_one(text)?;
    let part_two = solve_part_two(text)?;

    println!("part one: {part_one}");
    println!("part two: {part_two}");
    Ok(())
}
