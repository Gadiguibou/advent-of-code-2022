use anyhow::bail;
use num::Integer;

struct Monkey {
    starting_items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisible_by: u64,
    if_true_monkey_number: usize,
    if_false_monkey_number: usize,
}

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    let monkey_descriptions = input.split("\n\n");

    let mut monkeys = Vec::new();

    // Parse input
    for monkey_description in monkey_descriptions {
        let mut lines = monkey_description.lines();

        // Skip first line (Monkey id)
        lines.next();
        // Starting items
        let (_, item_list) = lines.next().unwrap().split_once(": ").unwrap();
        let starting_items: Vec<u64> = item_list.split(", ").map(|s| s.parse().unwrap()).collect();
        // Operation
        let (_, operation_string) = lines.next().unwrap().split_once("new = ").unwrap();
        let operation: Box<dyn Fn(u64) -> u64> =
            match operation_string.split_whitespace().collect::<Vec<_>>()[..] {
                ["old", "+", "old"] => Box::new(|old: u64| old + old),
                ["old", "+", value] => Box::new(|old| old + value.parse::<u64>().unwrap()),
                ["old", "*", "old"] => Box::new(|old| old * old),
                ["old", "*", value] => Box::new(|old| old * value.parse::<u64>().unwrap()),
                _ => bail!("Unknown operation: {}", operation_string),
            };
        // Test
        let (_, divisible_by_string) = lines.next().unwrap().split_once("divisible by ").unwrap();
        let divisible_by = divisible_by_string.parse::<u64>().unwrap();
        // If true
        let (_, if_true_monkey_number_string) = lines.next().unwrap().split_once("throw to monkey ").unwrap();
        let if_true_monkey_number = if_true_monkey_number_string.parse::<usize>().unwrap();
        // If false
        let (_, if_false_monkey_number_string) = lines.next().unwrap().split_once("throw to monkey ").unwrap();
        let if_false_monkey_number = if_false_monkey_number_string.parse::<usize>().unwrap();

        monkeys.push(Monkey {
            starting_items,
            operation,
            divisible_by,
            if_true_monkey_number,
            if_false_monkey_number,
        });
    }

    let mut monkey_activity = vec![0u64; monkeys.len()];
    let lcm = monkeys.iter().fold(1, |acc, monkey| Integer::lcm(&acc, &monkey.divisible_by));

    // 20 rounds for part 1, 10_000 for part 2
    for _round in 0..10000 {
        for monkey_number in 0..monkeys.len() {
            for &item in &monkeys[monkey_number].starting_items.clone() {
                let monkey = &monkeys[monkey_number];

                monkey_activity[monkey_number] += 1;

                // Monkey inspects item
                let new_item = (monkey.operation)(item);
                // Monkey gets bored (Uncomment for part 1)
                // let new_item = new_item / 3;
                // Modulo by LCM of all monkey's `divisible_by` to keep the numbers small without affecting tests
                let new_item = new_item % lcm;
                // Monkey throws item
                let condition_met = new_item % monkey.divisible_by == 0;
                let next_monkey_number = if condition_met {
                    monkey.if_true_monkey_number
                } else {
                    monkey.if_false_monkey_number
                };
                monkeys[next_monkey_number].starting_items.push(new_item);
            }
            // Monkey threw all items (assuming items are never thrown to the monkey that threw them)
            monkeys[monkey_number].starting_items.clear();
        }
    }

    monkey_activity.sort();
    monkey_activity.reverse();

    // Compute monkey business
    let monkey_business = monkey_activity[0] * monkey_activity[1];
    println!("Result: {}", monkey_business);

    Ok(())
}
