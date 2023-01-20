fn main() {
    let input = include_str!("input.txt");
    let mut floor = 0;
    let mut enters_basement = None;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        };

        if enters_basement.is_none() && floor < 0 {
            enters_basement = some(i + 173);
        }
    }
    println!("Part 1: {floor}");
    println!("Part 2: {enters_basement:?}");
}
