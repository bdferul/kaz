// inspired by https://www.reddit.com/r/adventofcode/comments/zifqmh/comment/izrd7iz/?utm_source=share&utm_medium=web2x&context=3

const INPUT: &str = include_str!("input.txt");

fn main() {
    let monkey_operations = parse_param(2, 23).collect::<Vec<String>>();
    let monkey_test = parse_param(3, 21)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i128>>();
    let monkey_conditions = parse_param(4, 29)
        .zip(parse_param(5, 30))
        .map(|(a, b)| [a, b])
        .map(|a| a.into_iter().map(|s| s.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>();

    let mut modulo = 1;
    for i in &monkey_test {
        modulo *= *i;
    }

    //def main(part)

    let monkey_inspections = vec![0u128; monkey_test.len()];
    let monkey_items = parse_param(1, 18)
        .map(parse_starting_items)
        .collect::<Vec<Vec<i128>>>();

    let mut results = [0; 2];
    for part in 1..=2 {
        let mut monkey_items = monkey_items.clone();
        let mut monkey_inspections = monkey_inspections.clone();
        for _ in 0..if part == 1 { 20 } else { 10000 } {
            for i in 0..monkey_inspections.len() {
                for j in 0..monkey_items[i].len() {
                    let mut current = monkey_items[i][j];
                    let current_op = monkey_operations[i].clone();
                    if current_op == "* old" {
                        current *= current;
                    } else {
                        let rhs: i128 = current_op[2..].parse().unwrap();
                        match &current_op[..1] {
                            "*" => current *= rhs,
                            "+" => current += rhs,
                            other => panic!("{other} is not valid"),
                        }
                    }

                    if part == 1 {
                        current /= 3;
                    } else {
                        current %= modulo;
                    }

                    if current % monkey_test[i] == 0 {
                        monkey_items[monkey_conditions[i][0]].push(current);
                    } else {
                        monkey_items[monkey_conditions[i][1]].push(current);
                    }
                    monkey_inspections[i] += 1;
                }
                monkey_items[i].clear();
            }
        }
        monkey_inspections.sort();
        println!("{:?}", &monkey_inspections);
        let mut mir = monkey_inspections.iter().rev();
        results[part - 1] = mir.next().unwrap() * mir.next().unwrap();
    }

    for (i, r) in results.into_iter().enumerate() {
        println!("Part {}: {}", i + 1, r);
    }
}

fn parse_starting_items(s: String) -> Vec<i128> {
    s.split(", ")
        .map(|s| s.parse().expect("Unable to parse starting items"))
        .collect()
}

fn parse_param(line_skip: usize, char_skip: usize) -> impl Iterator<Item = String> {
    INPUT
        .lines()
        .skip(line_skip)
        .step_by(7)
        .map(move |s| s.split_at(char_skip).1.trim().to_string())
}
