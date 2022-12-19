use anyhow::bail;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    let mut cycle_number = 0;
    let mut register_x = 1;
    let mut part_1 = 0;

    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();

        match parts[..] {
            ["noop"] => { increment_clock(&mut cycle_number, register_x, &mut part_1) }
            ["addx", value] => {
                let value = value.parse::<i32>()?;
                increment_clock(&mut cycle_number, register_x, &mut part_1);
                increment_clock(&mut cycle_number, register_x, &mut part_1);
                register_x += value;
            }
            _ => { bail!("Unknown instruction: {:?}", parts); }
        }
    }

    println!("Part 1: {part_1}");

    Ok(())
}

fn increment_clock(cycle_number: &mut i32, register_x: i32, part_1: &mut i32) {
    *cycle_number += 1;

    // Increment part 1 on significant cycles
    if *cycle_number >= 20 && (*cycle_number - 20) % 40 == 0 {
        let signal_strength = register_x * *cycle_number;
        *part_1 += signal_strength;
        // println!("Cycle: {cycle_number}, Register X: {register_x}, Signal strength: {signal_strength}");
    }

    // Draw pixel
    let current_pixel = *cycle_number - 1;
    if (current_pixel % 40 - register_x).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }

    // New line every 40 cycles
    if *cycle_number % 40 == 0 {
        println!();
    }
}
