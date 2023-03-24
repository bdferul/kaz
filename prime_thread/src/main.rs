use std::{
    ops::Div,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let start = 2;
    let arc_primes = Arc::new(Mutex::new(primes_under(start * start)));

    let (prod, recv) = std::sync::mpsc::channel();

    for a in start..start + 4 {
        let b = arc_primes.lock().unwrap().last().cloned().unwrap();
        thread::scope(|s| {
            for count in a * a..b * b {
                dbg!(count);
                if count > 1_000_000 {
                    break;
                }
                let arc_primes_clone = arc_primes.clone();
                let prod = prod.clone();
                s.spawn(move || {
                    let mut is_prime = true;
                    for p in arc_primes_clone.lock().unwrap().iter() {
                        if count % p == 0 {
                            is_prime = false;
                            break;
                        }
                        if p * p > count {
                            break;
                        }
                    }
                    prod.send((count, is_prime)).unwrap();
                });
            }
        });

        let mut v = recv
            .try_iter()
            .filter_map(|(p, b)| b.then_some(p))
            .collect::<Vec<u128>>();

        v.sort();

        let mut primes = arc_primes.lock().unwrap();
        primes.extend(v.into_iter());
        println!("{a}: {}", primes.len());
    }

    eprintln!("{arc_primes:?}");
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
