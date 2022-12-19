fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");
    let tree_grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.bytes().map(|b| b - b'0').collect())
        .collect();

    let mut visibility_grid: Vec<Vec<bool>> =
        vec![vec![false; tree_grid[0].len()]; tree_grid.len()];

    for row_index in 0..tree_grid.len() {
        let row = &tree_grid[row_index];

        // Mark trees visible from the left
        let mut left_max_height = 0u8;
        for col_index in 0..tree_grid[0].len() {
            let height = row[col_index];

            // All trees on the edge of the tree grid are visible
            if col_index == 0 || col_index == tree_grid[0].len() - 1 {
                visibility_grid[row_index][col_index] = true;
            }

            // If the tree is taller than the tallest tree to the left, it is visible
            if height > left_max_height {
                visibility_grid[row_index][col_index] = true;
                left_max_height = height;
            }
        }

        // Mark trees visible from the right
        let mut right_max_height = 0u8;
        for col_index in (0..tree_grid[0].len()).rev() {
            let height = row[col_index];

            // If the tree is taller than the tallest tree to the right, it is visible
            if height > right_max_height {
                visibility_grid[row_index][col_index] = true;
                right_max_height = height;
            }
        }
    }

    for col_index in 0..tree_grid[0].len() {
        // Mark trees visible from the top
        let mut top_max_height = 0u8;
        for row_index in 0..tree_grid.len() {
            let height = tree_grid[row_index][col_index];

            // All trees on the edge of the tree grid are visible
            if row_index == 0 || row_index == tree_grid.len() - 1 {
                visibility_grid[row_index][col_index] = true;
            }

            // If the tree is taller than the tallest tree to the top, it is visible
            if height > top_max_height {
                visibility_grid[row_index][col_index] = true;
                top_max_height = height;
            }
        }

        // Mark trees visible from the bottom
        let mut bottom_max_height = 0u8;
        for row_index in (0..tree_grid.len()).rev() {
            let height = tree_grid[row_index][col_index];

            // If the tree is taller than the tallest tree to the bottom, it is visible
            if height > bottom_max_height {
                visibility_grid[row_index][col_index] = true;
                bottom_max_height = height;
            }
        }
    }

    // Print the forest :)
    for row in &visibility_grid {
        for &visible in row {
            print!("{}", if visible { '#' } else { '.' });
        }
        println!();
    }

    // Count the number of visible trees
    let part_1 = visibility_grid.iter().flatten().filter(|&&b| b).count();

    // Determine the maximum scenic score
    let mut part_2 = 0;

    for row_index in 0..tree_grid.len() {
        for col_index in 0..tree_grid[0].len() {
            let scenic_score = calculate_scenic_score(&tree_grid, row_index, col_index);
            if scenic_score > part_2 {
                part_2 = scenic_score;
            }
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");

    Ok(())
}

fn calculate_scenic_score(tree_grid: &Vec<Vec<u8>>, row_index: usize, col_index: usize) -> usize {

    // Trees on the edge of the grid always have at least one direction with no visible trees
    // Hence, their scenic score is always 0.
    if row_index == 0
        || row_index == tree_grid.len() - 1
        || col_index == 0
        || col_index == tree_grid[0].len() - 1
    {
        return 0;
    }

    let height = tree_grid[row_index][col_index];

    let mut scenic_score = 1;

    // Count visible trees to the left
    let mut visible_trees = 1;
    let mut current_col = col_index - 1;
    while tree_grid[row_index][current_col] < height {
        if current_col == 0 {
            break;
        }
        visible_trees += 1;
        current_col -= 1;
    }

    scenic_score *= visible_trees;

    // Count visible trees to the right
    let mut visible_trees = 1;
    let mut current_col = col_index + 1;
    while tree_grid[row_index][current_col] < height {
        if current_col == tree_grid[0].len() - 1 {
            break;
        }
        visible_trees += 1;
        current_col += 1;
    }

    scenic_score *= visible_trees;

    // Count visible trees to the top
    let mut visible_trees = 1;
    let mut current_row = row_index - 1;
    while tree_grid[current_row][col_index] < height {
        if current_row == 0 {
            break;
        }
        visible_trees += 1;
        current_row -= 1;
    }

    scenic_score *= visible_trees;

    // Count visible trees to the bottom
    let mut visible_trees = 1;
    let mut current_row = row_index + 1;
    while tree_grid[current_row][col_index] < height {
        if current_row == tree_grid.len() - 1 {
            break;
        }
        visible_trees += 1;
        current_row += 1;
    }

    scenic_score *= visible_trees;

    scenic_score
}
