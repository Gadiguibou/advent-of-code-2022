fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    // Hardcoded initial stack height and number of stacks
    const INITIAL_STACK_HEIGHT: usize = 8;
    const NUMBER_OF_STACKS: usize = 9;

    let mut stacks: Vec<Vec<char>> = vec![vec![]; NUMBER_OF_STACKS];

    let stack_lines = input.lines().take(INITIAL_STACK_HEIGHT).collect::<Vec<_>>();

    for level in stack_lines.iter().rev() {
        for (stack_index, stack) in stacks.iter_mut().enumerate() {
            let index_in_line = 1 + stack_index * 4;
            let value = level.chars().nth(index_in_line).unwrap();
            if value != ' ' {
                stack.push(value);
            }
        }
    }

    let mut part_1_stacks = stacks.clone();
    let mut part_2_stacks = stacks;

    let instruction_lines = input
        .lines()
        .skip(INITIAL_STACK_HEIGHT + 2)
        .collect::<Vec<_>>();

    for instruction_line in instruction_lines {
        let mut split_on_whitespace = instruction_line.split_whitespace();
        let items_to_move = split_on_whitespace
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let starting_stack = split_on_whitespace
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;

        let ending_stack = split_on_whitespace
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap()
            - 1;

        // Instruction execution for part 1
        for _ in 0..items_to_move {
            let item = part_1_stacks[starting_stack].pop().unwrap();
            part_1_stacks[ending_stack].push(item);
        }

        // Instruction execution for part 2
        let range_to_move = part_2_stacks[starting_stack].len() - items_to_move..;
        let items: Vec<char> = part_2_stacks[starting_stack].drain(range_to_move).collect();
        part_2_stacks[ending_stack].extend(items);
    }

    let part_1: String = part_1_stacks.iter().map(|stack| stack[stack.len() - 1]).collect();
    let part_2: String = part_2_stacks.iter().map(|stack| stack[stack.len() - 1]).collect();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}
