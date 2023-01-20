/// An over-engineered solution to the common interview question: FizzBuzz
///
/// Allows for an arbitrary number of parameters following the format `(u32,String)`
///
/// # Examples
///
/// - Create using the default parameters:
///
/// ```
/// # use fizzbuzz::fizz_buzz_iter::FizzBuzzIter;
/// let mut fizzbuzz = FizzBuzzIter::default();
///
/// assert_eq!(fizzbuzz.next().unwrap(), "1");
/// assert_eq!(fizzbuzz.next().unwrap(), "2");
/// assert_eq!(fizzbuzz.next().unwrap(), "Fizz");
/// assert_eq!(fizzbuzz.nth(1).unwrap(), "Buzz");// 5
/// assert_eq!(fizzbuzz.nth(9).unwrap(), "FizzBuzz");// 15
/// ```
///
/// - Create with custom parameters:
///
/// ```
/// # use fizzbuzz::fizz_buzz_iter::FizzBuzzIter;
/// let params = vec![(7, "Kazz"), (11, "Gozz")];
/// let mut fizzbuzz = FizzBuzzIter::from(params);
///
/// assert_eq!(fizzbuzz.to_answer(7*11), "KazzGozz");
/// ```
///
/// - Add parameters
///
/// ```
/// # use fizzbuzz::fizz_buzz_iter::FizzBuzzIter;
/// let mut fizzbuzz = FizzBuzzIter::default();
/// fizzbuzz.add_param(7, "Kazz");
/// // Now tests 3 5, and 7
/// assert_eq!(fizzbuzz.to_answer(3*5*7), "FizzBuzzKazz");
/// ```
///
/// - Use in a loop
///
/// ```
/// # use fizzbuzz::fizz_buzz_iter::FizzBuzzIter;
/// let fizzbuzz = FizzBuzzIter::default();
///
/// for fb in fizzbuzz.take(5) {
///     println!("{fb}");
/// }
/// ```
/// ```text
/// OUTPUT:
/// 1
/// 2
/// Fizz
/// 4
/// Buzz
/// ```
#[derive(Clone)]
pub struct FizzBuzzIter {
    params: Vec<(u32, String)>,
    count: u32,
}

impl FizzBuzzIter {
    /// Calculates the answer based on which paramaters the given value is divisible by
    ///
    /// ```
    /// # use fizzbuzz::fizz_buzz_iter::FizzBuzzIter;
    /// let fizzbuzz = FizzBuzzIter::default();
    ///
    /// assert_eq!(fizzbuzz.to_answer(15), "FizzBuzz");
    /// // 15 is divisible by both 3 and 5
    /// ```
    pub fn to_answer(&self, value: u32) -> String {
        let mut truths = vec![];
        for (test, string) in &self.params {
            if value % test == 0 {
                truths.push(string);
            }
        }

        if truths.is_empty() {
            value.to_string()
        } else {
            truths
                .into_iter()
                .fold(String::new(), |acc, s| format!("{}{}", acc, s))
        }
    }

    /// Allows for the addition of a paramater
    /// # Example
    /// ```
    /// # use fizzbuzz::fizz_buzz_iter::FizzBuzzIter;
    /// let mut fizzbuzz = FizzBuzzIter::default();
    /// fizzbuzz.add_param(7, "Kazz");
    /// // Now tests 3 5, and 7
    /// assert_eq!(fizzbuzz.to_answer(3*5*7), "FizzBuzzKazz");
    /// ```
    pub fn add_param(&mut self, test: u32, string: &str) {
        self.params.push((test, string.to_string()));
    }
}

impl From<Vec<(u32, String)>> for FizzBuzzIter {
    fn from(value: Vec<(u32, String)>) -> Self {
        Self {
            params: value,
            count: 1,
        }
    }
}

impl From<Vec<(u32, &str)>> for FizzBuzzIter {
    fn from(value: Vec<(u32, &str)>) -> Self {
        Self::from(
            value
                .into_iter()
                .map(|(n, s)| (n, s.to_string()))
                .collect::<Vec<_>>(),
        )
    }
}

impl Default for FizzBuzzIter {
    /// Returns the default as expected by the problem description where:
    ///
    /// Any number divisible by 3 is replaced with "Fizz", 5 with "Buzz", or both with "FizzBuzz"
    fn default() -> Self {
        FizzBuzzIter::from(vec![(3, "Fizz"), (5, "Buzz")])
    }
}

impl Iterator for FizzBuzzIter {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.to_answer(self.count - 1))
    }
}
