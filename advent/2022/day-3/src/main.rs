use std::vec;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let alphabet = ('a'..='z').chain('A'..='Z');
    /*Part 1
    let out: usize = INPUT.lines().map(|l| {
        let (s1,s2) = l.split_at(l.len()/2);
        s1.chars().find(|c| s2.contains(*c)).unwrap()
    }).map(|c| alphabet.iter().position(|&x| x == c).unwrap() + 1).sum();*/
    let mut out = 0;
    let lines = INPUT.lines().collect::<Vec<&str>>();
    let mut groups = vec![];
    let group_size = 3;
    for i in (0..lines.len()).step_by(group_size) {
        groups.push(lines[i..i + group_size].to_vec());
    }
    for elves in groups.iter() {
        let e2 = elves[1].chars().filter(|&c| elves[0].contains(c)).collect::<String>();
        let e3 = elves[2].chars().find(|&c| e2.contains(c)).unwrap();
        out += alphabet.clone().position(|c| c==e3).unwrap() + 1;
    }
    println!("{out}");
}
