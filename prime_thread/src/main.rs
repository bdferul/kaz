use std::{thread, time::Duration};

fn main() {
    let start = 100;
    let mut primes = primes_under(start * start);

    for a in start..start + 1 {
        let b = a + 1;
        thread::scope(|s| {
            let mut threads = vec![];
            for count in a * a..b * b {
                threads.push(s.spawn(move || println!("{}", count)));
            }
        })
    }
}

fn factor(x: &u128) -> Vec<u128> {
    let mut pre = vec![];
    let mut post = vec![];

    for i in 1.. {
        let (q, r) = (x / i, x % i);
        if q < i {
            break;
        }
        if r == 0 {
            pre.push(i);
            if q != i {
                post.push(q);
            }
        }
    }

    pre.into_iter().chain(post.into_iter().rev()).collect()
}

/// returns a vector all prime numbers under the given maximum
fn primes_under(max: u128) -> Vec<u128> {
    let mut primes = vec![2];

    for count in (3..max).step_by(2) {
        let mut is_prime = true;
        for p in &primes {
            if count % p == 0 {
                is_prime = false;
                break;
            }

            if p * p > count {
                break;
            }
        }

        if is_prime {
            primes.push(count);
        }
    }

    primes
}
