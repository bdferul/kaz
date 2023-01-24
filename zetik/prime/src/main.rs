use clap::Parser;
use zetik_prime::PrimeIter;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Will only show prime numbers after this input
    #[arg(short, long, default_value = "1834")]
    start: u64,

    /// Will only show prime numbers after this input
    #[arg(short, long, default_value = "0")]
    after: u64,

    /// How many values will be printed
    #[arg(short, long, default_value = "7")]
    range: u64,
}

fn main() {
    let opt = Args::parse();
    let print_fmt = |id, next| println!("{id}> +{} {next} ", next - opt.start);

    let mut primes = PrimeIter::default();
    primes.last_where(|x| x < opt.start + opt.after);
    for i in 1..=opt.range {
        let next = primes.next().unwrap();
        print_fmt(i, next);
    }
}
