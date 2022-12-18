fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let input_chars = input.bytes().collect::<Vec<_>>();

    // Part 1
    for (index, window) in input_chars.windows(4).enumerate() {
        // Represent the presence of each character in the window as a bit in a u32 (pos = 1 << position in alphabet)
        let mut bits = 0u32;
        for c in window {
            bits |= 1 << (c - b'a');
        }
        if bits.count_ones() == 4 {
            println!("Part 1: {}", index + 4);
            break;
        }
    }

    // Part 2
    // Represent the number of each character in the window as an array of u8
    let mut counts = [0u8; 26];
    for index in 0..input_chars.len() {
        // Decrement the count of the character leaving the window
        if index >= 14 {
            counts[(input_chars[index - 14] - b'a') as usize] -= 1;
        }

        // Increment the count of the character entering the window
        counts[(input_chars[index] - b'a') as usize] += 1;

        // Check if the window contains 14 distinct characters
        if counts.iter().filter(|&&c| c > 0).count() == 14 {
            println!("Part 2: {}", index + 1);
            break;
        }
    }

    Ok(())
}
