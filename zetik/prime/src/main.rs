const GEN_START: u64 = 2093;
const PRINT_START: u64 = 2000;
const PRINT_LEN: u64 = 7;

use zetik_prime::PrimeIter;

fn main() {
    let print_fmt = |id, next: u64| println!("{id}> +{} {next} ", next.saturating_sub(GEN_START));

    let mut primes = PrimeIter::default();
    primes.last_where(|x| x < PRINT_START || x < GEN_START);
    for i in 1..=PRINT_LEN {
        let next = primes.next().unwrap();
        print_fmt(i, next);
    }
}
