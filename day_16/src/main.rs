use std::collections::{HashMap, VecDeque};

use prse::try_parse;

fn main() -> anyhow::Result<()> {
    // Would a greedy algorithm work here?
    // It seems like it wouldn't necessarily:
    // What if picking a suboptimal first valve to open led to opening two other valves that were better than the first one?

    // Maybe possible solutions can be viewed as a tree of paths. All edges have a "cost" of 1 minute to traverse, but
    // some of them have a value > 0 (when opening a valve but staying in the same room). The best path is the one with
    // the highest value.
    // Then, the number of paths is O(d^30) where d is the max number of tunnels from a valve. In our case d = 5.

    // To prevent allocating the whole tree since we're only visiting it once, we can use a stack of current valve, time left, valves opened, pressure released.
    // When a path reaches its end (time runs out), we check if it's the best path so far and save its value if it is.

    // 5^30 is still way too much (10^20).
    // We don't have to visit all paths, only those that are on the shortest path to an unopened valve with some non-zero flow rate.
    // Unfortunately computing those shortest paths may be quite long too, but let's try anyway.
    // If all valves with non-zero flow rates are already opened we can just add the current flow rate * time left to the pressure released.

    let input = include_str!("../example.txt");

    // Since there are fewer than 64 valves, we can represent whether they were visited using a u64.
    let mut current_id: usize = 0;
    let mut valve_ids: HashMap<&str, usize> = HashMap::new();

    let valves = input
        .lines()
        .map(|line| {
            let (valve_id, flow_rate, tunnels): (&str, usize, Vec<&str>) = try_parse!(
                line,
                "Valve {} has flow rate={}; tunnels lead to valves {:, :}"
            )
            .or(try_parse!(
                line,
                "Valve {} has flow rate={}; tunnel leads to valve {:, :}"
            ))
            .unwrap();

            valve_ids.insert(valve_id, current_id);
            current_id += 1;

            (valve_id, (flow_rate, tunnels))
        })
        .collect::<HashMap<_, _>>();

    // Valves with non-zero flows
    let valves_with_flow = valves
        .iter()
        .filter(|(_, (flow, _))| *flow > 0)
        .map(|(valve, _)| *valve)
        .collect::<Vec<_>>();

    // Current valve, time left, valves opened (bitmap), pressure released until now, pressure released during the next time step
    let mut stack = vec![("AA", 30, 0, 0, 0)];
    let mut part_1 = 0;

    let mut iterations = 0;

    while let Some((current_valve, time_left, valves_opened, pressure_released, flow)) = stack.pop()
    {
        iterations += 1;
        if iterations % 10_000_000 == 0 {
            println!("Iterations: {}", iterations);
        }
        let time_left = time_left - 1;

        if time_left == 0 {
            if pressure_released > part_1 {
                part_1 = pressure_released;
                println!("New best: {}", part_1);
            }
            continue;
        }

        let (current_valve_flow, tunnels) = &valves[current_valve];

        let mut work_left = false;

        // Choose to move to another valve
        let mut tunnels_used = 0u64;

        for valve in &valves_with_flow {
            if valves_opened & (1 << valve_ids[valve]) != 0 {
                continue;
            }

            if let Some(next_valve) = shortest_path(&valves, &valve_ids, current_valve, valve) {
                let valve_id = valve_ids[next_valve];
                if tunnels_used & (1 << valve_id) != 0 {
                    continue;
                }

                tunnels_used |= 1 << valve_id;

                stack.push((
                    next_valve,
                    time_left,
                    valves_opened,
                    pressure_released + flow,
                    flow,
                ));

                work_left = true;
            }

        }

        // for tunnel in tunnels {
        //     stack.push((
        //         tunnel,
        //         time_left,
        //         valves_opened,
        //         pressure_released + flow,
        //         flow,
        //     ))
        // }

        // Choose to open the current valve
        let valve_id = valve_ids[current_valve];
        if valves_opened & (1 << valve_id) == 0 {
            stack.push((
                current_valve,
                time_left,
                valves_opened | (1 << valve_id),
                pressure_released + flow,
                flow + current_valve_flow,
            ));

            work_left = true;
        }

        if !work_left {
            let total_pressure_released = pressure_released + flow * time_left;
            if total_pressure_released > part_1 {
                part_1 = total_pressure_released;
                println!("New best: {}", part_1);
            }
        }
    }

    println!("Part 1: {}", part_1);

    Ok(())
}

// Returns the first valve on the shortest path to the destination valve.
fn shortest_path(
    valves: &HashMap<&str, (usize, Vec<&'static str>)>,
    valve_ids: &HashMap<&str, usize>,
    current_valve: &'static str,
    destination_valve: &'static str,
) -> Option<&'static str> {
    // (first_valve, current_valve)
    let (_, tunnels) = &valves[current_valve];
    let mut queue = tunnels
        .iter()
        .map(|valve| (valve, valve))
        .collect::<VecDeque<_>>();
    let mut visited = 0u64; // bitmap

    while let Some((first_valve, current_valve)) = queue.pop_front() {
        let valve_id = valve_ids[current_valve];

        if visited & (1 << valve_id) != 0 {
            continue;
        }

        visited |= 1 << valve_id;

        if current_valve == &destination_valve {
            return Some(first_valve);
        }

        let (_, tunnels) = &valves[current_valve];

        for tunnel in tunnels {
            queue.push_back((first_valve, tunnel));
        }
    }
    None
}
