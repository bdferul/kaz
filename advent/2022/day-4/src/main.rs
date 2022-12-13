use std::ops::Range;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut result = 0;
    let splits = INPUT.lines()
        .map(|l| l.split_once(',').unwrap())
        .map(|(a,b)| [a,b]);
    for split in splits {
        let mut range = split.iter().map(|&x| {
            let (a,b) = x.split_once('-').unwrap();
            let v = [a,b].into_iter().map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
            v[0]..v[1]
        }).collect::<Vec<Range<i32>>>();
        
        range.sort_by(|a,b| a.start.partial_cmp(&b.start).unwrap());

        //if range[0].end >= range[1].end || range[0].start == range[1].start { //Part 1
        if range[0].end >= range[1].start { //Part 2
            result += 1;
        }
    }
    println!("{result}");
}
