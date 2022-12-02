use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = std::fs::read_to_string("input.txt")?;

    let input = input_file.trim().split("\n\n").map(|s| s.lines());

    let calories_per_elf: Vec<u64> = input
        .clone()
        .map(|lines| -> u64 {
            lines
                .map(|s| -> u64 {
                    s.parse()
                        .unwrap_or_else(|_| panic!("Failed to parse line {s}"))
                })
                .sum()
        }).collect();

    let part_1 = calories_per_elf.iter().max().expect("No max found");

    println!("Part 1: {part_1}");

    let mut top_three = [0, 0, 0];

    for calories in calories_per_elf {
        let mut pos = 0;
        while pos < 3 && calories < top_three[pos] {
            pos += 1;
        }
        if pos < 3 {
            let mut i = 2;
            while i > pos {
                top_three[i] = top_three[i - 1];
                i -= 1;
            }
            top_three[pos] = calories;
        }
    }

    let part_2: u64 = top_three.iter().sum();

    println!("Part 2: {part_2}");

    Ok(())
}
