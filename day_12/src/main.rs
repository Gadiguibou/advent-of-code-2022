use std::collections::VecDeque;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    let mut heightmap: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    // Find starting and ending point
    let starting_row = heightmap
        .iter()
        .position(|row| row.contains(&b'S'))
        .unwrap();
    let starting_col = heightmap[starting_row]
        .iter()
        .position(|&c| c == b'S')
        .unwrap();
    let ending_row = heightmap
        .iter()
        .position(|row| row.contains(&b'E'))
        .unwrap();
    let ending_col = heightmap[ending_row]
        .iter()
        .position(|&c| c == b'E')
        .unwrap();

    let starting_point = (starting_row, starting_col);

    let ending_point = (ending_row, ending_col);

    // Set correct height of starting and ending point
    heightmap[starting_row][starting_col] = b'a';
    heightmap[ending_row][ending_col] = b'z';

    // Number of steps to get from 'S' to 'E'
    let part_1 = shortest_path(&heightmap, starting_point, ending_point).unwrap();

    // Find all positions with height 'a'
    let positions_with_height_a = heightmap.iter().enumerate().flat_map(|(row_index, row)| {
        row.iter().enumerate().filter_map(move |(col_index, &c)| {
            if c == b'a' {
                Some((row_index, col_index))
            } else {
                None
            }
        })
    });

    let part_2 = positions_with_height_a
        .filter_map(|position| shortest_path(&heightmap, position, ending_point))
        .min()
        .unwrap();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

fn can_step(starting_height: u8, ending_height: u8) -> bool {
    ending_height as i8 - starting_height as i8 <= 1
}

fn shortest_path(
    heightmap: &Vec<Vec<u8>>,
    starting_point: (usize, usize),
    ending_point: (usize, usize),
) -> Option<usize> {
    // Use BFS to find the shortest path
    // Queue contains the number of steps from the start to a position and the position
    let mut queue: VecDeque<(usize, (usize, usize))> = VecDeque::new();
    queue.push_back((0, starting_point));

    // Set of visited positions
    let mut visited: Vec<Vec<bool>> = vec![vec![false; heightmap[0].len()]; heightmap.len()];

    while let Some((distance, next_position)) = queue.pop_front() {
        let (row, col) = next_position;

        // If we have reached the end, we are done
        if next_position == ending_point {
            return Some(distance);
        }

        // If we have already visited this position, we can skip it
        if visited[row][col] {
            continue;
        }

        visited[row][col] = true;

        // Check all possible directions
        for (new_row, new_col) in &[
            (row as isize - 1, col as isize),
            (row as isize + 1, col as isize),
            (row as isize, col as isize - 1),
            (row as isize, col as isize + 1),
        ] {
            // Check if the new position is inside the grid
            if *new_row < 0
                || *new_row >= heightmap.len() as isize
                || *new_col < 0
                || *new_col >= heightmap[0].len() as isize
            {
                continue;
            }

            let new_row = *new_row as usize;
            let new_col = *new_col as usize;

            // Check if we can step from the current position to the new position
            if can_step(heightmap[row][col], heightmap[new_row][new_col]) {
                queue.push_back((distance + 1, (new_row, new_col)));
            }
        }
    }
    None
}
