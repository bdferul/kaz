#![warn(clippy::nursery, clippy::complexity)]

//! # zetik_prime
//!
//! A library primarily dedicated to generating prime numbers with an iterator
//!
//! ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠿⠿⠿⠿⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⣿⣿⠟⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠉⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⣿⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢺⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠆⠜⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⠿⠿⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠻⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⡏⠁⠀⠀⠀⠀⠀⣀⣠⣤⣤⣶⣶⣶⣶⣶⣦⣤⡄⠀⠀⠀⠀⢀⣴⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣷⣄⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⢿⡧⠇⢀⣤⣶⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⣾⣮⣭⣿⡻⣽⣒⠀⣤⣜⣭⠐⢐⣒⠢⢰⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⣿⣏⣿⣿⣿⣿⣿⣿⡟⣾⣿⠂⢈⢿⣷⣞⣸⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⣿⣿⣽⣿⣿⣷⣶⣾⡿⠿⣿⠗⠈⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠻⠋⠉⠑⠀⠀⢘⢻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⣿⡿⠟⢹⣿⣿⡇⢀⣶⣶⠴⠶⠀⠀⢽⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⣿⣿⣿⡿⠀⠀⢸⣿⣿⠀⠀⠣⠀⠀⠀⠀⠀⡟⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⣿⣿⣿⡿⠟⠋⠀⠀⠀⠀⠹⣿⣧⣀⠀⠀⠀⠀⡀⣴⠁⢘⡙⢿⣿⣿⣿⣿⣿⣿⣿⣿<br/>
//! ⠉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠈⠙⢿⠗⠂⠄⠀⣴⡟⠀⠀⡃⠀⠉⠉⠟⡿⣿⣿⣿⣿<br/>
//! ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢷⠾⠛⠂⢹⠀⠀⠀⢡⠀⠀⠀⠀⠀⠙⠛⠿⢿<br/>

/// Provides an iterator for generating prime numbers.
///
/// ```
/// # use zetik_prime::PrimeIter;
/// let mut primes = PrimeIter::default();
/// assert_eq!(primes.next(), Some(2));
///
/// let next5: Vec<u64> = primes.take(5).collect();
/// assert_eq!(next5, [3, 5, 7, 11, 13]);
/// ```
#[must_use = "iterators are lazy and do nothing unless consumed"]
#[derive(Default, PartialEq, Eq)]
pub struct PrimeIter {
    primes: Vec<u64>,
}

impl PrimeIter {
    /// Returns the prime number after the given argument.
    ///
    /// Return value is an option to keep inline with `<Iterator>.next()`
    ///
    /// ```
    /// # use zetik_prime::PrimeIter;
    /// let mut res = PrimeIter::default().next_after(1000);
    /// assert_eq!(res, 1009);
    /// ```
    pub fn next_after(&mut self, num: u64) -> u64 {
        loop {
            let next = self.next().unwrap();
            if next > num {
                break next;
            }
        }
    }

    /// Returns the last value where the given condition is true, and sets the iterator at that value.
    ///
    /// ```
    /// # use zetik_prime::PrimeIter;
    /// let mut res = PrimeIter::default().last_where(|x| x <= 100).unwrap();
    ///
    /// assert_eq!(res, 97);
    /// ```
    pub fn last_where<F>(&mut self, f: F) -> Option<u64>
    where
        F: Fn(<Self as Iterator>::Item) -> bool,
    {
        loop {
            let next = self.next().unwrap();
            if !f(next) {
                self.primes.pop();
                break self.primes.last().cloned();
            }
        }
    }

    /// Returns `true` if no primes have been generated
    ///
    /// ```
    /// # use zetik_prime::PrimeIter;
    /// let mut primes = PrimeIter::default();
    /// assert_eq!(primes.is_empty(), true);
    /// primes.next();
    /// assert_eq!(primes.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.primes.is_empty()
    }

    /// Returns the length of the inner `Vec<u64>`
    ///
    /// ```
    /// # use zetik_prime::PrimeIter;
    /// let mut primes = PrimeIter::default();
    /// primes.nth(100);
    /// assert_eq!(primes.len(), 101);
    /// ```
    pub fn len(&self) -> usize {
        self.primes.len()
    }
}

/// Returns the prime factors of the given argument.
///
/// ```
/// # use zetik_prime::prime_factors;
/// assert_eq!(prime_factors(126), [2, 3, 3, 7]);
/// assert_eq!(prime_factors(12345), [3, 5, 823]);
/// assert_eq!(prime_factors(2 * 3 * 5 * 7 * 11), [2, 3, 5, 7, 11]);
/// ```
pub fn prime_factors(num: u64) -> Vec<u64> {
    let mut facts = vec![];
    let mut num = num;

    for i in 2.. {
        while num % i == 0 {
            num /= i;
            facts.push(i);
        }

        if num == 1 {
            break;
        }
    }

    facts
}

impl Iterator for PrimeIter {
    type Item = u64;

    /// Returns the next prime number.
    fn next(&mut self) -> Option<Self::Item> {
        let new = if let Some(&last) = self.primes.last() {
            let mut a = if last < 3 { 3 } else { last + 2 };
            for cnt in (a..).step_by(2) {
                let mut is_prime = true;
                let cnt_sqr = f32::sqrt(cnt as f32) as u64;
                for &p in &self.primes {
                    if cnt % p == 0 {
                        is_prime = false;
                        break;
                    }
                    if p > cnt_sqr {
                        break;
                    }
                }

                if is_prime {
                    a = cnt;
                    break;
                }
            }
            a
        } else {
            2
        };

        self.primes.push(new);
        Some(new)
    }
}

impl core::fmt::Debug for PrimeIter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.primes)
    }
}
