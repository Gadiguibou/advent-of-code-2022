use std::collections::HashSet;

use anyhow::bail;
use prse::parse;

fn main() -> anyhow::Result<()> {
    let input = include_str!("../input.txt");

    let lines = input.lines();

    let sensor_beacon_pairs = lines.map(|line| {
        let (sensor_x, sensor_y, beacon_x, beacon_y): (isize, isize, isize, isize) = parse!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}"
        );

        ((sensor_x, sensor_y), (beacon_x, beacon_y))
    });

    // Part 1
    let mut covered_cells_in_row_2_000_000 = HashSet::new();
    let mut beacons_in_row_2_000_000 = HashSet::new();

    for ((sensor_x, sensor_y), (beacon_x, beacon_y)) in sensor_beacon_pairs.clone() {
        if beacon_y == 2_000_000 {
            beacons_in_row_2_000_000.insert(beacon_x);
        }

        let distance = manhattan_distance(sensor_x, sensor_y, beacon_x, beacon_y);

        // Cells where another beacon cannot beacon can be found in the following way
        // At y = sensor_y, cells within [sensor_x - distance, sensor_x + distance] cannot contain a beacon
        // At y = sensor_y +/- distance, cells within [sensor_x, sensor_x] cannot contain a beacon
        // In general, at y = sensor_y +/- d, cells within [sensor_x - (distance - d), sensor_x + (distance - d)] cannot contain a beacon
        // In the above, if distance - d < 0 then the range is empty

        // Part 1
        // Find all cells that cannot contain a beacon in row 2_000_000
        let d = (2_000_000 - sensor_y).abs();
        if d >= 0 {
            for x in sensor_x - (distance - d)..=sensor_x + (distance - d) {
                covered_cells_in_row_2_000_000.insert(x);
            }
        }
    }

    // `-` on `HashSet`s is the set difference operator
    let part_1 = (&covered_cells_in_row_2_000_000 - &beacons_in_row_2_000_000).len();

    println!("Part 1: {part_1}");

    // Part 2
    // Idea:
    // To find the first free cell quickly, when visiting a cell, if it is covered, skip to the end of the first zone
    // covered by the relevant sensor or the start of the next row.

    let sensor_distance_pairs = sensor_beacon_pairs
        .map(|((sensor_x, sensor_y), (beacon_x, beacon_y))| {
            let distance = manhattan_distance(sensor_x, sensor_y, beacon_x, beacon_y);

            ((sensor_x, sensor_y), distance)
        })
        .collect::<Vec<_>>();

    let mut x = 0;
    let mut y = 0;

    let free_cell = 'outer: loop {
        if y > 4_000_000 {
            bail!("No free cell found");
        }

        for &((sensor_x, sensor_y), distance_covered) in sensor_distance_pairs.iter() {
            let distance_to_sensor = manhattan_distance(sensor_x, sensor_y, x, y);

            if distance_to_sensor > distance_covered {
                continue;
            }

            // Cell is covered by sensor
            // Skip to the end of the zone covered by the sensor in this row or to the start of the next row
            let d = (sensor_y - y).abs();
            let last_covered_x_in_row = sensor_x + (distance_covered - d);

            if last_covered_x_in_row + 1 > 4_000_000 {
                // Next row
                x = 0;
                y += 1;
            } else {
                // Next zone
                x = last_covered_x_in_row + 1;
            }
            continue 'outer;
        }

        break (x, y);
    };

    let part_2 = free_cell.0 * 4_000_000 + free_cell.1;

    println!("Part 2: {part_2}");

    Ok(())
}

fn manhattan_distance(x1: isize, y1: isize, x2: isize, y2: isize) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}
