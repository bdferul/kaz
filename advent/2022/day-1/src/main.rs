const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut elves: Vec<i32> = INPUT.split("\n\n").map(|elf| elf.split('\n').fold(0, |acc, s| acc + s.parse::<i32>().unwrap_or(0))).collect();
    let mut top = vec![];
    for _ in 0..3 {
        let max = elves.clone().into_iter().max().unwrap();
        let max_p = elves.iter().position(|x| *x == max).unwrap();
        elves.remove(max_p);
        top.push(max);
    }
    for (i,x) in top.iter().enumerate() {
        println!("{}: {x}", i+1);
    }
    println!("Part 2: {}", top.iter().sum::<i32>());
}
