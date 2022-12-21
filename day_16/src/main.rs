use anyhow::{anyhow, bail};
use itertools::Itertools;

use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
};

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

    // What if we instead start by testing every ordering of the valves with non-zero flow rates and choose the shortest path through them?
    // That would be O(v!) where v is the number of valves with non-zero flow rates.
    // v = 14 in our case, so that's still achievable although very slow (14! = 87_178_291_200).

    // 

    let input = include_str!("../input.txt");

    // There are fewer than 64 valves so we can use a bitset to represent maps of booleans to valves

    let mut valve_ids = HashMap::new();
    let mut current_id = 0;

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

    let part_1 = valves_with_flow
        .iter()
        .permutations(valves_with_flow.len())
        .map(|valves_to_open| {
            let valves_to_open = valves_to_open
                .iter()
                .map(|valve| **valve)
                .collect::<Vec<_>>();

            let path = shortest_path(&valves, "AA", &valves_to_open).unwrap();

            // dbg!(&path);
            // dbg!(evaluate_path(&valves, &valves_to_open, &path));

            evaluate_path(&valves, &valves_to_open, &path)
        })
        .max()
        .unwrap();

    println!("Part 1: {part_1}");

    Ok(())
}

std::thread_local! {
    static SHORTEST_PATHS: RefCell<HashMap<(&'static str, &'static str),Vec<&'static str>>>  = RefCell::new(HashMap::new());
}

fn bfs(
    valves: &HashMap<&'static str, (usize, Vec<&'static str>)>,
    starting_valve: &'static str,
    destination_valve: &'static str,
) -> Option<Vec<&'static str>> {
    if let Some(path) = SHORTEST_PATHS.with(|paths| {
        paths
            .borrow()
            .get(&(starting_valve, destination_valve))
            .cloned()
    }) {
        return Some(path);
    }

    // BFS
    let mut queue = VecDeque::new();
    queue.push_back(vec![starting_valve]);

    while let Some(path) = queue.pop_front() {
        let current_valve = path.last().unwrap();

        if current_valve == &destination_valve {
            SHORTEST_PATHS.with(|paths| {
                paths
                    .borrow_mut()
                    .insert((starting_valve, destination_valve), path.clone());
            });
            return Some(path);
        }

        let (_, tunnels) = valves
            .get(current_valve)
            .unwrap_or_else(|| panic!("Valve {current_valve} not found"));

        for tunnel in tunnels {
            let mut new_path = path.clone();
            new_path.push(tunnel);
            queue.push_back(new_path);
        }
    }

    None
}

// Shortest path through a sequence of valves
fn shortest_path(
    valves: &HashMap<&'static str, (usize, Vec<&'static str>)>,
    starting_valve: &'static str,
    valves_to_visit: &[&'static str],
) -> Option<Vec<&'static str>> {
    let mut path = vec![starting_valve];
    let mut time_required = 0;

    for valve in valves_to_visit {
        let new_path = bfs(valves, path.last().unwrap(), valve)?;

        time_required += path.len();

        if time_required > 30 {
            return Some(path);
        }

        // Remove the last valve since it's also in the new path
        path.pop();
        path.extend(new_path);
    }

    Some(path)
}

// Evaluate final pressure released for a given path
fn evaluate_path(
    valves: &HashMap<&'static str, (usize, Vec<&'static str>)>,
    valves_to_open: &[&'static str],
    path: &[&'static str],
) -> usize {
    let mut pressure_released = 0;
    let mut current_flow = 0;
    let mut time_elapsed = 0;
    let mut next_valve_index = 0;
    let mut path_index = 0;

    while time_elapsed < 30 && path_index < path.len() && next_valve_index < valves_to_open.len() {
        time_elapsed += 1;
        pressure_released += current_flow;

        if path[path_index] == valves_to_open[next_valve_index] {
            let (flow, _) = valves.get(valves_to_open[next_valve_index]).unwrap();
            current_flow += flow;
            next_valve_index += 1;
        } else {
            path_index += 1;
        }
    }

    if time_elapsed < 30 {
        pressure_released += current_flow * (30 - time_elapsed);
    }

    pressure_released
}
