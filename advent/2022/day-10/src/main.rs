const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut reg_x = 1;
    let mut cycles = vec![];

    for l in INPUT.lines() {
        let mut words = l.split_whitespace();

        match words.next().unwrap() {
            "noop" => cycles.push(reg_x),
            "addx" => {
                for _ in 0..2 {
                    cycles.push(reg_x)
                }
                reg_x += words.next().unwrap().parse::<i32>().unwrap()
            }
            o => panic!("{o}"),
        }
    }

    let range_max = [cycles.len(), 220].into_iter().min().unwrap();
    println!(
        "Part 1: {}",
        (19..range_max)
            .step_by(40)
            .map(|i| cycles[i] * (i as i32 + 1))
            .sum::<i32>()
    );

    print!("Part 2:\t");
    for (cycle,x) in cycles.into_iter().enumerate() {
        let sprite = x..x+3;
        let pos = cycle as i32 % 40;
        if cycle > 0 && pos == 0 {
            print!("\n\t");
        }

        let print_char = if sprite.contains(&(pos + 1)) {
            '#'
        } else {
            '.'
        };

        print!("{print_char}");
    }
    println!(); 
}
