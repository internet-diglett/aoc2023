use std::collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap,
};

use anyhow::{anyhow, Result};

type GameData<'a> = (u64, Vec<Vec<(u64, &'a str)>>);

///
/// ```txt
/// The Elf would first like to know which games would have been possible
/// if the bag contained only 12 red cubes, 13 green cubes, and 14 blue cubes?
/// ```
/// return `true` iff a given number and falls within the permitted ranges
///
fn allowed_for_part_one(number: u64, color: &str) -> bool {
    match (number, color) {
        (n, "red") if n <= 12 => true,
        (n, "green") if n <= 13 => true,
        (n, "blue") if n <= 14 => true,
        _ => false,
    }
}

///
/// ```txt
/// ...once a bag has been loaded with cubes, the Elf will reach into the bag,
/// grab a handful of random cubes, show them to you, and then put them back
/// in the bag. He'll do this a few times per game.
/// ```
///
/// parse each line (game) into the individual pieces of information
/// needed to perform the calculations required for solving the puzzle.
///
fn parse_line(text: &str) -> Result<GameData> {
    // drop the "Game" prefix from the data
    let (_, useful_text) = text
        .split_once(' ')
        .ok_or(anyhow!("malformatted line, no space separated data"))?;

    // split the game id from the rest of the data
    let (id, draw_data) = useful_text
        .split_once(':')
        .ok_or(anyhow!("malformatted line, no colon separated data"))?;

    let parsed_id: u64 = id.parse()?;

    // break the remaining data into the subsets
    // ["3 blue, 4 red", "1 red, 2 green", ...]
    let subsets = draw_data.split(';');

    // this vec will hold the data representing the final format
    // [[("3", "blue"), ("4", "red")], [("1", "red"), ("2", "green")], ...]
    let mut parsed_subsets: Vec<Vec<(u64, &str)>> = vec![];

    // Since the str::split we called above returned an iterator and not a Vec / slice,
    // the actual split operation is being performed while we loop here, so we're not
    // losing performance by iterating over the string data multiple times.
    for subset in subsets {
        // lets break the subset into strings indicating number and color
        // i.e. "3 blue, 4 red" => ["3 blue", "4 red"]
        let cube_data = subset.split(',');

        // this vec will hold the
        let mut parsed_cube_data: Vec<(u64, &str)> = vec![];

        // again, the str::split(',') we called a few lines ago didn't actually perform
        // the split operation, but instead waited until we began iterating over the str,
        // gifting us additional performance.
        for data in cube_data {
            // lets break the number and color strings into tuples
            // i.e. "3 blue" =>  (3, "blue")
            let (count, color) = data
                .trim()
                .split_once(' ')
                .ok_or(anyhow!("malformatted line, dice data not space separated"))?;

            let parsed_count: u64 = count.parse()?;
            parsed_cube_data.push((parsed_count, color));
        }

        parsed_subsets.push(parsed_cube_data);
    }
    Ok((parsed_id, parsed_subsets))
}

fn highest_count_seen(data: &GameData) -> HashMap<String, u64> {
    let mut counts: HashMap<String, u64> = HashMap::new();
    let (_, sets) = data;

    for set in sets {
        for (count, color) in set {
            match counts.entry(color.to_string()) {
                Occupied(mut entry) => {
                    // update logic
                    let value = entry.get_mut();
                    if *value < *count {
                        *value = *count;
                    }
                }
                Vacant(entry) => {
                    entry.insert(*count);
                    // do the insert
                }
            }
        }
    }
    counts
}

fn possible_game(counts: HashMap<String, u64>, within_rules: fn(u64, &str) -> bool) -> bool {
    for (color, count) in counts {
        if !within_rules(count, &color) {
            return false;
        }
    }
    true
}

///
/// ```txt
/// Determine which games would have been possible if the bag had been
/// loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes.
/// What is the sum of the IDs of those games?
/// ```
///
/// ```
/// use day2::solve_part_one;
/// use std::fs;
///
/// let text = fs::read_to_string("src/part1_example.txt").unwrap();
/// let result = solve_part_one(&text).unwrap();
/// assert_eq!(result, 8)
/// ```
///
pub fn solve_part_one(text: &str) -> Result<u64> {
    let mut game_ids: Vec<u64> = vec![];
    // for each line in game data
    for line in text.lines() {
        // parse game data
        let data = parse_line(line)?;
        // find highest counts seen
        let counts = highest_count_seen(&data);
        // record id if it is a valid game based on the rules
        if possible_game(counts, allowed_for_part_one) {
            game_ids.push(data.0);
        }
    }

    // sum ids
    Ok(game_ids.into_iter().sum())
}

///
/// ```txt
/// As you continue your walk, the Elf poses a second question: in each game you played,
/// what is the fewest number of cubes of each color that could have been in the bag to
/// make the game possible?
///
/// In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes.
/// If any color had even one fewer cube, the game would have been impossible.
/// Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
/// Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
/// Game 4 required at least 14 red, 3 green, and 15 blue cubes.
/// Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.
///
/// The power of a set of cubes is equal to the numbers of red, green, and blue cubes
/// multiplied together. The power of the minimum set of cubes in game 1 is 48.
/// In games 2-5 it was 12, 1560, 630, and 36, respectively. Adding up these five powers
/// produces the sum 2286.
///
/// For each game, find the minimum set of cubes that must have been present.
/// What is the sum of the power of these sets?
/// ```
///
/// ```
/// use day2::solve_part_two;
/// use std::fs;
///
/// let text = fs::read_to_string("src/part1_example.txt").unwrap();
/// let result = solve_part_two(&text).unwrap();
/// assert_eq!(result, 2286)
/// ```
///
pub fn solve_part_two(text: &str) -> Result<u64> {
    let mut game_powers: Vec<u64> = vec![];
    // for each line in game data
    for line in text.lines() {
        // parse game data
        let data = parse_line(line)?;
        // find highest counts seen
        let counts = highest_count_seen(&data);
        // calculate the powers
        let power = counts.values().product::<u64>();
        game_powers.push(power);
    }

    // sum powers
    Ok(game_powers.into_iter().sum())
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

    fn game_data() -> GameData<'static> {
        (
            1,
            vec![
                vec![(3, "blue"), (4, "red")],
                vec![(1, "red"), (2, "green"), (6, "blue")],
                vec![(2, "green")],
            ],
        )
    }

    #[test]
    fn should_parse_line() -> Result<()> {
        let text = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = game_data();
        let result = parse_line(text)?;
        Ok(assert_eq!(result, expected))
    }

    #[test]
    fn should_find_highest_count_seen() {
        let data = game_data();
        let expected = HashMap::from([
            ("blue".to_string(), 6),
            ("red".to_string(), 4),
            ("green".to_string(), 2),
        ]);
        let result = highest_count_seen(&data);
        assert_eq!(result, expected)
    }

    #[test]
    fn should_find_possible_game() {
        let possible_game_data = game_data();
        let good_count = highest_count_seen(&possible_game_data);
        let result = possible_game(good_count, allowed_for_part_one);
        assert!(result);

        let impossible_game_data = (
            1,
            vec![
                vec![(1000, "blue"), (4, "red")],
                vec![(1, "red"), (2, "green"), (6, "blue")],
                vec![(2, "green")],
            ],
        );
        let bad_count = highest_count_seen(&impossible_game_data);
        let result = possible_game(bad_count, allowed_for_part_one);
        assert!(!result);
    }
}
