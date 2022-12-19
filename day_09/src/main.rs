use std::collections::HashSet;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    // Part 1
    let mut positions_visited_part_1: HashSet<(i32, i32)> = HashSet::new();

    let mut tail_position = (0, 0);
    let mut head_position = (0, 0);

    // Part 2
    let mut positions_visited_part_2: HashSet<(i32, i32)> = HashSet::new();
    let mut rope_positions: [(i32, i32); 10] = [(0, 0); 10];

    for line in input.lines() {
        let (direction, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<i32>()?;

        let (dx, dy) = match direction {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => panic!("Unknown direction"),
        };

        for _ in 0..steps {
            // Part 1
            head_position.0 += dx;
            head_position.1 += dy;

            move_tail(head_position, &mut tail_position);

            positions_visited_part_1.insert(tail_position);

            // Part 2
            rope_positions[0].0 += dx;
            rope_positions[0].1 += dy;

            for i in 1..rope_positions.len() {
                move_tail(rope_positions[i - 1], &mut rope_positions[i]);
            }

            positions_visited_part_2.insert(rope_positions[rope_positions.len() - 1]);
        }
    }

    let part_1 = positions_visited_part_1.len();
    let part_2 = positions_visited_part_2.len();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

fn move_tail(head_position: (i32, i32), tail_position: &mut (i32, i32)) {
    let tail_dx = head_position.0 - tail_position.0;
    let tail_dy = head_position.1 - tail_position.1;
    // up and right
    if tail_dx > 1 && tail_dy > 0 || tail_dy > 1 && tail_dx > 0 {
        tail_position.0 += 1;
        tail_position.1 += 1;
    }
    // down and right
    else if tail_dx > 1 && tail_dy < 0 || tail_dy < -1 && tail_dx > 0 {
        tail_position.0 += 1;
        tail_position.1 -= 1;
    }
    // down and left
    else if tail_dx < -1 && tail_dy < 0 || tail_dy < -1 && tail_dx < 0 {
        tail_position.0 -= 1;
        tail_position.1 -= 1;
    }
    // up and left
    else if tail_dx < -1 && tail_dy > 0 || tail_dy > 1 && tail_dx < 0 {
        tail_position.0 -= 1;
        tail_position.1 += 1;
    }
    // right
    else if tail_dx > 1 {
        tail_position.0 += 1;
    }
    // left
    else if tail_dx < -1 {
        tail_position.0 -= 1;
    }
    // up
    else if tail_dy > 1 {
        tail_position.1 += 1;
    }
    // down
    else if tail_dy < -1 {
        tail_position.1 -= 1;
    }
}
