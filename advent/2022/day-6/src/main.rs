fn main() {
    let input = include_str!("input.txt");
    println!("input: {input}");
    
    println!("Part 1: {}", problem(input, 4));
    println!("Part 1: {}", problem(input, 14));
}

///sliding window!
fn problem(str: &str, len: usize) -> i32 {
    let mut count = len - 1;
    for i in 0..(str.len() - len) {
        count += 1;
        let pattern = &str[i..i+len];
        if has_unique_elements(pattern.chars()) {
            break;
        }
    }

    count as i32
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + std::hash::Hash,
{
    let mut uniq = std::collections::HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}