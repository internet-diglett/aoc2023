use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap, HashSet,
};

use anyhow::{anyhow, Context, Result};

///
/// ```txt
/// The Elf leads you over to the pile of colorful cards.
/// There, you discover dozens of scratchcards, all with
/// their opaque covering already scratched off. Picking one up,
/// it looks like each card has two lists of numbers separated by
/// a vertical bar (|): a list of winning numbers and then a list
/// of numbers you have. You organize the information into a
/// table (your puzzle input).
///
/// As far as the Elf has been able to figure out, you have to
/// figure out which of the numbers you have appear in the list of
/// winning numbers. The first match makes the card worth one point
/// and each match after the first doubles the point value of that card.
///
/// For example:
///
/// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
///
/// In the above example, card 1 has five winning numbers
/// (41, 48, 83, 86, and 17) and eight numbers you have
/// (83, 86, 6, 31, 17, 9, 48, and 53). Of the numbers you have,
/// four of them (48, 83, 17, and 86) are winning numbers! That
/// means card 1 is worth 8 points (1 for the first match, then
/// doubled three times for each of the three matches after the first).
///
///     Card 2 has two winning numbers (32 and 61), so it is worth 2 points.
///     Card 3 has two winning numbers (1 and 21), so it is worth 2 points.
///     Card 4 has one winning number (84), so it is worth 1 point.
///     Card 5 has no winning numbers, so it is worth no points.
///     Card 6 has no winning numbers, so it is worth no points.
///
/// So, in this example, the Elf's pile of scratchcards is worth 13 points.
/// ```
///
/// ```
/// use day4::solve_part_one;
/// use std::fs;
///
/// let text = fs::read_to_string("src/part1_example.txt").unwrap();
/// let result = solve_part_one(&text).unwrap();
/// assert_eq!(result, 13)
/// ```
///
pub fn solve_part_one(text: &str) -> Result<u64> {
    let mut total_points = 0;

    for line in text.lines() {
        // split card prefix
        let (_id, useful_text) = line
            .split_once(':')
            .ok_or(anyhow!("malformatted line, no colon separated data"))?;

        // split list of numbers
        let (winning_numbers, our_numbers) = useful_text
            .split_once('|')
            .ok_or(anyhow!("malformatted line, no '|' separated data"))?;

        let winning_numbers: Vec<u64> = winning_numbers
            .split_ascii_whitespace()
            .map(|number| number.parse::<u64>().map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<u64>>>()?;

        let winning_numbers: HashSet<u64> = HashSet::from_iter(winning_numbers);

        let our_numbers: Vec<u64> = our_numbers
            .split_ascii_whitespace()
            .map(|number| number.parse::<u64>().map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<u64>>>()?;

        let number_of_matches = our_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();

        if number_of_matches > 0 {
            // points is (matches - 1) to the power of 2
            let card_points = 1 << (number_of_matches - 1);
            total_points += card_points;
        }
    }

    Ok(total_points)
}

///
/// ```txt
/// This time, the above example goes differently:
///
/// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
/// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
/// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
/// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
/// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
/// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
///
///     Card 1 has four matching numbers, so you win one copy each of
///     the next four cards: cards 2, 3, 4, and 5.
///     Your original card 2 has two matching numbers, so you win
///     one copy each of cards 3 and 4.
///     Your copy of card 2 also wins one copy each of cards 3 and 4.
///     Your four instances of card 3 (one original and three copies) have
///     two matching numbers, so you win four copies each of cards 4 and 5.
///     Your eight instances of card 4 (one original and seven copies) have
///     one matching number, so you win eight copies of card 5.
///     Your fourteen instances of card 5 (one original and thirteen copies)
///     have no matching numbers and win no more cards.
///     Your one instance of card 6 (one original) has no matching numbers and
///     wins no more cards.
///
/// Once all of the originals and copies have been processed, you end up with
/// 1 instance of card 1, 2 instances of card 2, 4 instances of card 3, 8
/// instances of card 4, 14 instances of card 5, and 1 instance of card 6.
/// In total, this example pile of scratchcards causes you to ultimately
/// have 30 scratchcards!
/// ```
///
/// ```
/// use day4::solve_part_two;
/// use std::fs;
///
/// let text = fs::read_to_string("src/part1_example.txt").unwrap();
/// let result = solve_part_two(&text).unwrap();
/// assert_eq!(result, 30)
/// ```
///
pub fn solve_part_two(text: &str) -> Result<u64> {
    let mut card_counts: HashMap<usize, usize> = HashMap::new();
    let mut lines = text.lines().peekable();
    let mut sum: u64 = 0;

    while let Some(line) = lines.next() {
        // split card prefix
        let (id, useful_text) = line
            .split_once(':')
            .ok_or(anyhow!("malformatted line, no colon separated data"))?;

        // split number from card id
        let (_, card_number) = id.split_once(' ').ok_or(anyhow!("malformatted card id"))?;
        let card_number = card_number
            .trim()
            .parse()
            .with_context(|| "failed to parse card number")?;

        match card_counts.entry(card_number) {
            Occupied(mut existing_entry) => {
                *existing_entry.get_mut() += 1;
            }
            Vacant(new_entry) => {
                new_entry.insert(1);
            }
        }

        // split list of numbers
        let (winning_numbers, our_numbers) = useful_text
            .split_once('|')
            .ok_or(anyhow!("malformatted line, no '|' separated data"))?;

        let winning_numbers: Vec<u64> = winning_numbers
            .split_ascii_whitespace()
            .map(|number| number.parse::<u64>().map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<u64>>>()?;

        let winning_numbers: HashSet<u64> = HashSet::from_iter(winning_numbers);

        let our_numbers: Vec<u64> = our_numbers
            .split_ascii_whitespace()
            .map(|number| number.parse::<u64>().map_err(|e| anyhow!(e)))
            .collect::<Result<Vec<u64>>>()?;

        let number_of_matches = our_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .count();

        for i in 1..=number_of_matches {
            let card_to_increment = card_number + i;
            let value = match card_counts.get(&card_number) {
                Some(n) => *n,
                None => 1,
            };
            match card_counts.entry(card_to_increment) {
                Occupied(mut existing_entry) => {
                    *existing_entry.get_mut() += value;
                }
                Vacant(new_entry) => {
                    new_entry.insert(value);
                }
            }
        }
        if lines.peek().is_none() {
            // we've reached the last line. Drop all hashmap keys that are greater
            // than this card number, as they do not exist and should not count towards
            // our totals
            sum = card_counts
                .iter()
                .filter(|(k, _)| **k <= card_number)
                .map(|(_, v)| *v as u64)
                .sum();
        }
    }
    Ok(sum)
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
}
