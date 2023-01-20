use zetik_prime::PrimeIter;

fn main() {
    let mut args = std::env::args();
    let input_start: u64 = args
        .nth(1)
        .unwrap_or_else(|| "0".to_string())
        .parse()
        .unwrap();
    let input_after: u64 = args
        .next()
        .unwrap_or_else(|| "0".to_string())
        .parse()
        .unwrap();
    let input_range: usize = args
        .next()
        .unwrap_or_else(|| "5".to_string())
        .parse()
        .unwrap();

    let print_fmt = |id, next| println!("{id}> +{} {next} ", next - input_start);

    let mut primes = PrimeIter::default();
    primes.last_where(|x| x < input_start + input_after);
    for i in 1..=input_range {
        let next = primes.next().unwrap();
        print_fmt(i, next);
    }

    /*
    let next_after = primes.next_after(input_start).unwrap();
    print_fmt(primes.len(), next_after);
    for i in 2..=input_range {
        let next = primes.next().unwrap();
        print_fmt(primes.len(), next);
    }
    */
}
