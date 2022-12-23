use std::{collections::HashMap, time::Instant};

use prse::{parse, try_parse};

fn main() -> anyhow::Result<()> {
    // Use dynamic programming to solve this
    let input = include_str!("../input.txt");

    let valves: HashMap<_, _> = input
        .lines()
        .map(|line| {
            let (name, flow_rate, tunnels): (&str, usize, Vec<&str>) = try_parse!(
                line,
                "Valve {} has flow rate={}; tunnels lead to valves {:, :}"
            )
            .unwrap_or_else(|_| {
                parse!(
                    line,
                    "Valve {} has flow rate={}; tunnel leads to valve {:, :}"
                )
            });

            (name, (flow_rate, tunnels))
        })
        .collect();

    // Map valve names to their index in the valves vector to avoid using &str to index into the memoization table
    let valve_ids: HashMap<&str, usize> = valves
        .iter()
        .enumerate()
        .map(|(index, (&name, _))| (name, index))
        .collect();

    let valves: Vec<(u32, Vec<usize>)> = valves
        .into_iter()
        .map(|(_name, (flow_rate, tunnels))| {
            (
                flow_rate as u32,
                tunnels.into_iter().map(|name| valve_ids[name]).collect(),
            )
        })
        .collect();

    // Used to index into the open valves bitmap
    let ids_of_valves_with_flow = valves
        .iter()
        .enumerate()
        .filter(|(_, (flow_rate, _))| *flow_rate > 0)
        .enumerate()
        .map(|(new_index, (original_index, _))| (original_index, new_index))
        .collect::<HashMap<_, _>>();

    // Make sure we don't overflow the bitmap and that we generate the right amount of open_valves bitmaps
    assert!(ids_of_valves_with_flow.len() == 15);

    // Part 1
    println!("Part 1");

    // Memoization table
    // Maps [time left: 31][open valves: 2^15][you: 61] -> max pressure released
    println!("Initializing memoization table... This can take a while");
    let mut memo = vec![[[0u32; 61]; 2usize.pow(15)]; 31];
    println!("Done initializing memoization table");

    let start_time = Instant::now();
    // Skip time 0 since the result is always 0
    for time_left in 1..=30 {
        for open_valves in 0..2usize.pow(15) {
            for current_valve in 0..61 {
                let (flow_rate, tunnels) = &valves[current_valve];

                let mut best = 0;

                // Possibilities at each step:
                // - You move through a tunnel
                // - You open a valve

                // You move through a tunnel
                for &tunnel in tunnels {
                    let pressure_released = memo[time_left - 1][open_valves][tunnel];

                    best = best.max(pressure_released);
                }

                let can_open_valve = *flow_rate > 0
                    && open_valves & 1 << ids_of_valves_with_flow[&current_valve] == 0;

                // You open a valve
                if can_open_valve {
                    let open_valves = open_valves | 1 << ids_of_valves_with_flow[&current_valve];

                    let pressure_released = *flow_rate * (time_left - 1) as u32
                        + memo[time_left - 1][open_valves][current_valve];

                    best = best.max(pressure_released);
                }

                memo[time_left][open_valves][current_valve] = best;
            }
        }
        println!("{time_left}/30");
    }

    let part_1 = memo[30][0][valve_ids["AA"]];
    println!("Part 1: {part_1}");
    println!("Completed part 1 in {} milliseconds", start_time.elapsed().as_millis());

    // Part 2
    println!("Part 2");

    // Memoization table
    // Maps [time left: 27][open valves: 2^15][you: 61][elephant: 61] -> max pressure released
    // let mut memo = [[[[0; 61]; 61]; 2usize.pow(15)]; 27];
    println!("Initializing memoization table... This can take a while");
    let mut memo = vec![vec![[[0u32; 61]; 61]; 2usize.pow(15)]; 27];
    println!("Done initializing memoization table");

    let start_time = Instant::now();

    // Skip time 0 since the result is always 0
    for time_left in 1..=26 {
        for open_valves in 0..2usize.pow(15) {
            for you in 0..61 {
                for elephant in 0..61 {
                    // (you, elephant) has the same value as (elephant, you)
                    if you > elephant {
                        memo[time_left][open_valves][you][elephant] =
                            memo[time_left][open_valves][elephant][you];
                        continue;
                    }

                    // You
                    let (flow_rate, tunnels) = &valves[you];
                    // Elephant
                    let (elephant_flow_rate, elephant_tunnels) = &valves[elephant];

                    let mut best = 0;

                    // Possibilities at each step:
                    // - You and the elephant move through a tunnel
                    // - You and the elephant open a valve (make sure its not the same)
                    // - You open a valve and the elephant moves through a tunnel
                    // - You move through a tunnel and the elephant opens a valve

                    // You and the elephant move through a tunnel
                    for &tunnel in tunnels {
                        for &elephant_tunnel in elephant_tunnels {
                            let pressure_released =
                                memo[time_left - 1][open_valves][tunnel][elephant_tunnel];

                            best = best.max(pressure_released);
                        }
                    }

                    let you_can_open_valve =
                        *flow_rate > 0 && open_valves & 1 << ids_of_valves_with_flow[&you] == 0;
                    let elephant_can_open_valve = *elephant_flow_rate > 0
                        && open_valves & 1 << ids_of_valves_with_flow[&elephant] == 0;

                    // You and the elephant open a valve
                    if you_can_open_valve && elephant_can_open_valve && you != elephant {
                        let open_valves = open_valves
                            | 1 << ids_of_valves_with_flow[&you]
                            | 1 << ids_of_valves_with_flow[&elephant];

                        let pressure_released = *flow_rate * (time_left - 1) as u32
                            + *elephant_flow_rate * (time_left - 1) as u32
                            + memo[time_left - 1][open_valves][you][elephant];

                        best = best.max(pressure_released);
                    }

                    // You open a valve and the elephant moves through a tunnel
                    if you_can_open_valve {
                        let open_valves = open_valves | 1 << ids_of_valves_with_flow[&you];

                        for &elephant_tunnel in elephant_tunnels {
                            let pressure_released = *flow_rate * (time_left - 1) as u32
                                + memo[time_left - 1][open_valves][you][elephant_tunnel];

                            best = best.max(pressure_released);
                        }
                    }

                    // You move through a tunnel and the elephant opens a valve
                    if elephant_can_open_valve {
                        let open_valves = open_valves | 1 << ids_of_valves_with_flow[&elephant];

                        for &tunnel in tunnels {
                            let pressure_released = *elephant_flow_rate * (time_left - 1) as u32
                                + memo[time_left - 1][open_valves][tunnel][elephant];

                            best = best.max(pressure_released);
                        }
                    }

                    memo[time_left][open_valves][you][elephant] = best;
                }
            }
        }
        println!("{time_left}/26");
    }

    let part_2 = memo[26][0][valve_ids["AA"]][valve_ids["AA"]];

    println!("Part 2: {part_2}");
    println!("Completed part 2 in {} seconds", start_time.elapsed().as_secs());

    Ok(())
}
