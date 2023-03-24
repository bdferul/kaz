use std::collections::{btree_map::ValuesMut, HashMap};

const INPUT: &str = include_str!("input.txt");
const TIME_LIMIT: i64 = 30;
const STARTING_VALVE: &str = "AA";
const SKIP_IF: i32 = 3;

#[derive(Default, Debug)]
struct Valve {
    label: String,
    flow_rate: i32,
    leads_to: Vec<String>,
    open: bool,
}

fn main() {
    let mut valves = HashMap::new();

    for valve in parse_input(INPUT) {
        valves.insert(valve.label.clone(), valve);
    }

    let mut part1 = 0;
    let mut flow_rate = 0;
    let mut current_valve = STARTING_VALVE;

    for _ in 0..TIME_LIMIT {
        let valve = valves.get_mut(current_valve).unwrap();

        dbg!(&valve);

        if valve.flow_rate > SKIP_IF && !valve.open {
            valve.open = false;
            flow_rate += valve.flow_rate;
        } else {
        }

        part1 += flow_rate;
    }

    println!("Part 1: {part1}");
}

fn parse_input(input: &str) -> Vec<Valve> {
    let mut r = vec![];

    let splitters = [" ", " ", "=", ";", "valve", " "];

    for line in input.lines() {
        let mut splits = vec![];
        let mut line = line;

        for s in splitters {
            let Some((a,b)) = line.split_once(s) else { break };
            splits.push(a);
            line = b;
        }

        splits.push(line);

        let valve = Valve {
            label: splits[1].to_string(),
            flow_rate: splits[3].parse().unwrap(),
            leads_to: splits[6].split(',').map(|s| s.trim().to_string()).collect(),
            open: false,
        };

        r.push(valve);
    }

    r
}
