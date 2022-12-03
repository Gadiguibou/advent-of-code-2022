use std::{
    collections::{hash_map::RandomState, HashSet},
    error::Error,
};

use itertools::Itertools;

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 'a' as u32 + 1
    } else {
        c as u32 - 'A' as u32 + 27
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = std::fs::read_to_string("input.txt")?;
    let input_lines: Vec<&str> = input_file.trim().lines().collect();

    let mut part_1_total = 0;
    for line in input_lines.iter() {
        let parts = line.split_at(line.len() / 2);

        let part_1_chars: HashSet<char, RandomState> = HashSet::from_iter(parts.0.chars());
        let part_2_chars: HashSet<char, RandomState> = HashSet::from_iter(parts.1.chars());

        let mut common_chars = &part_1_chars & &part_2_chars;
        if common_chars.len() != 1 {
            panic!("Expected exactly one common char, got {:?}", common_chars);
        }

        let common_char = common_chars.drain().next().unwrap();

        part_1_total += priority(common_char);
    }

    println!("Part 1: {part_1_total}");

    let mut part_2_total = 0;
    for (rucksack_1, rucksack_2, rucksack_3) in input_lines.iter().tuples() {
        let rucksack_1_chars: HashSet<char, RandomState> = HashSet::from_iter(rucksack_1.chars());
        let rucksack_2_chars: HashSet<char, RandomState> = HashSet::from_iter(rucksack_2.chars());
        let rucksack_3_chars: HashSet<char, RandomState> = HashSet::from_iter(rucksack_3.chars());

        let mut common_chars = &((&rucksack_1_chars) & (&rucksack_2_chars)) & (&rucksack_3_chars);
        if common_chars.len() != 1 {
            panic!("Expected exactly one common char, got {:?}", common_chars);
        };

        let common_char = common_chars.drain().next().unwrap();

        part_2_total += priority(common_char);
    }

    println!("Part 2: {part_2_total}");

    Ok(())
}
