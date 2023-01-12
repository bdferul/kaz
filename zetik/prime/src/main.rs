use zetik_prime::Prime;

fn main() {
    let mut args = std::env::args();
    let input_start: u64 = args.nth(1).unwrap_or_else(|| "0".to_string()).parse().unwrap();
    let input_range: usize = args.next().unwrap_or_else(|| "5".to_string()).parse().unwrap();

    let mut primes = Prime::default();

    let next_after = primes.next_after(input_start).unwrap();
    println!("1> {next_after}: +{}", next_after - input_start);
    for i in 2..input_range + 1 {
        let next = primes.next().unwrap();
        println!("{i}> {next}: +{}", next - input_start);
    }
}
