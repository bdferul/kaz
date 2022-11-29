/// A collection of prime number operations
pub mod primes {
    /// returns a vector of primes that are less than the supplied argument
    pub fn primes_under(a: u32) -> Vec<u32> {
        let mut primes = vec![2];
        let mut count = 3;

        while count < a {
            let mut is_prime = true;
            for x in &primes {
                if count % x == 0 {
                    is_prime = false;
                    break;
                }

                if x * x > count {
                    break;
                }
            }
            if is_prime {
                primes.push(count);
            }
            count += 2;
        }

        primes
    }

    /// returns nth prime number or zero
    pub fn nth(a: u32) -> u32 {
        *primes_under(a).last().unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn primes_under() {
        assert_eq!(
            super::primes::primes_under(25),
            [2, 3, 5, 7, 11, 13, 17, 19, 23]
        )
    }
}
