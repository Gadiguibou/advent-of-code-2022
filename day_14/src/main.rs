use anyhow::bail;
use itertools::Itertools;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    let lines = input
        .lines()
        .map(|line| line.split(" -> ").map(|s| s.split_once(',').unwrap()));

    let mut grid = vec![vec![false; 1000]; 1000];

    let mut max_y = 0;

    for line in lines {
        for ((x1, y1), (x2, y2)) in line.tuple_windows() {
            let (mut x1, mut y1, mut x2, mut y2) = (
                x1.parse::<usize>()?,
                y1.parse::<usize>()?,
                x2.parse::<usize>()?,
                y2.parse::<usize>()?,
            );

            if x1 > x2 || y1 > y2 {
                (x1, y1, x2, y2) = (x2, y2, x1, y1)
            }

            max_y = max_y.max(y2);

            for x in x1..=x2 {
                for y in y1..=y2 {
                    grid[x][y] = true;
                }
            }
        }
    }


    // Part 1
    // Grains of sand coming to a rest before grains of sand go falling into the void

    let mut part_1_grid = grid.clone();
    let mut part_1 = 0;

    'outer: loop {
        let (mut grain_x, mut grain_y) = (500, 0);

        loop {
            if grain_y == 999 {
                // Grain has fallen to the bottom
                break 'outer;
            }

            if !part_1_grid[grain_x][grain_y + 1] {
                grain_y += 1;
                continue;
            }

            if grain_x == 0 {
                // Grain has fallen to the left
                break 'outer;
            }

            if !part_1_grid[grain_x - 1][grain_y + 1] {
                grain_x -= 1;
                grain_y += 1;
                continue;
            }

            if grain_x == 999 {
                // Grain has fallen to the right
                break 'outer;
            }

            if !part_1_grid[grain_x + 1][grain_y + 1] {
                grain_x += 1;
                grain_y += 1;
                continue;
            }

            // Grain comes to rest
            part_1_grid[grain_x][grain_y] = true;
            part_1 += 1;
            break;
        }
    }

    println!("Part 1: {part_1}");



    // Draw the floor at height max_y + 2

    // Part 2
    // Grains of sand coming to a rest before grains of sand go falling into the void

    let mut part_2_grid = grid.clone();
    let mut part_2 = 0;

    for row in &mut part_2_grid {
        row[max_y + 2] = true;
    }

    'outer: loop {
        let (mut grain_x, mut grain_y) = (500, 0);

        loop {
            if grain_y == 999 {
                // Grain has fallen to the bottom
                bail!("Grain has fallen to the bottom")
            }

            if !part_2_grid[grain_x][grain_y + 1] {
                grain_y += 1;
                continue;
            }

            if grain_x == 0 {
                // Grain has fallen to the left
                bail!("Grain has fallen to the left");
            }

            if !part_2_grid[grain_x - 1][grain_y + 1] {
                grain_x -= 1;
                grain_y += 1;
                continue;
            }

            if grain_x == 999 {
                // Grain has fallen to the right
                bail!("Grain has fallen to the right")
            }

            if !part_2_grid[grain_x + 1][grain_y + 1] {
                grain_x += 1;
                grain_y += 1;
                continue;
            }

            // Grain comes to rest
            part_2_grid[grain_x][grain_y] = true;
            part_2 += 1;
            if grain_x == 500 && grain_y == 0 {
                // Grain has reached the top
                break 'outer;
            }
            break;
        }
    }

    println!("Part 2: {part_2}");

    Ok(())
}
