// inspired by https://www.reddit.com/r/adventofcode/comments/zifqmh/comment/izrd7iz/?utm_source=share&utm_medium=web2x&context=3

const INPUT: &str = include_str!("input.txt");

#[derive(Clone)]
struct Monkey {
    items: Vec<i128>,
    operation: String,
    test: i128,
    conditions: Vec<usize>,
    inspections: u128,
}

impl Monkey {
    fn from_input(input: &str) -> Vec<Monkey> {
        let mut lines = input.lines();

        let mut monkeys = vec![];

        while lines.next().is_some() {
            monkeys.push(Monkey {
                items: lines.next().unwrap()[18..]
                    .trim()
                    .split(", ")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<i128>>(),
                operation: lines.next().unwrap()[23..].trim().to_string(),
                test: lines.next().unwrap()[21..].trim().parse().unwrap(),
                conditions: vec![
                    lines.next().unwrap()[29..].trim().parse().unwrap(),
                    lines.next().unwrap()[30..].trim().parse().unwrap(),
                ],
                inspections: 0,
            });
            lines.next();
        }

        monkeys
    }
}

fn main() {
    let monkeys = Monkey::from_input(INPUT);

    let mut modulo = 1;
    for i in monkeys.iter().map(|m| m.test) {
        modulo *= i;
    }

    let mut results = [0; 2];
    for part in 1..=2 {
        let mut monkeys = monkeys.clone();
        for _ in 0..if part == 1 { 20 } else { 10000 } {
            for i in 0..monkeys.len() {
                for j in 0..monkeys[i].items.len() {
                    if monkeys[i].operation == "* old" {
                        monkeys[i].items[j] *= monkeys[i].items[j];
                    } else {
                        let rhs: i128 = monkeys[i].operation[2..].parse().unwrap();
                        match &monkeys[i].operation[..1] {
                            "*" => monkeys[i].items[j] *= rhs,
                            "+" => monkeys[i].items[j] += rhs,
                            other => panic!("{other} is not valid"),
                        }
                    }

                    if part == 1 {
                        monkeys[i].items[j] /= 3;
                    } else {
                        monkeys[i].items[j] %= modulo;
                    }

                    let item = monkeys[i].items[j];
                    if monkeys[i].items[j] % monkeys[i].test == 0 {
                        let cnd = monkeys[i].conditions[0];
                        monkeys[cnd].items.push(item);
                    } else {
                        let cnd = monkeys[i].conditions[1];
                        monkeys[cnd].items.push(item);
                    }
                    monkeys[i].inspections += 1;
                }
                monkeys[i].items.clear();
            }
        }
        monkeys.sort_by_key(|m| m.inspections);
        let mut mir = monkeys.iter().rev();
        results[part - 1] = mir.next().unwrap().inspections * mir.next().unwrap().inspections;
    }

    for (i, r) in results.into_iter().enumerate() {
        println!("Part {}: {}", i + 1, r);
    }
}
