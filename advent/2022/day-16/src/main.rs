const INPUT: &str = include_str!("input.txt");

#[derive(Default, Debug)]
struct Valve {
    label: String,
    flow_rate: i32,
    leads_to: Vec<String>,
}

impl Valve {
    fn new() -> Self {
        Default::default()
    }

    fn label(self, input: &str) -> Self {
        Self {
            label: input.to_string(),
            ..self
        }
    }

    fn flow(self, flow_rate: i32) -> Self {
        Self { flow_rate, ..self }
    }

    fn leads_to(self, leads_to: Vec<String>) -> Self {
        Self { leads_to, ..self }
    }
}

fn main() {
    let valves = parse_input(INPUT);

    dbg!(valves);
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

        dbg!(&splits);

        let valve = Valve::new()
            .label(splits[1])
            .flow(splits[3].parse().unwrap())
            .leads_to(splits[6].split(',').map(|s| s.trim().to_string()).collect());

        r.push(valve);
    }

    r
}
