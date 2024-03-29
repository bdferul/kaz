use zetik_prime::Prime;

fn main() {
    let mut args = std::env::args();
    let input_start: u64 = args
        .nth(1)
        .unwrap_or_else(|| "0".to_string())
        .parse()
        .unwrap();
    let input_range: usize = args
        .next()
        .unwrap_or_else(|| "5".to_string())
        .parse()
        .unwrap();

    let mut x = 1..5;

    x.nth(0);

    let mut primes = Prime::default();

    let next_after = primes.next_after(input_start).unwrap();
    let print_fmt = |id, next| println!("{id}> {next} + {}", next - input_start);
    print_fmt(1, next_after);
    for i in 2..input_range + 1 {
        let next = primes.next().unwrap();
        print_fmt(i, next);
    }
}
