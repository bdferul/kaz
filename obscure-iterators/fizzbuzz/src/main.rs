use fizzbuzz::fizz_buzz_iter::FizzBuzzIter;

fn main() {
    let mut fb = FizzBuzzIter::default();
    fb.add_param(7, "Kazz");

    for s in fb.take(25) {
        println!("{s}");
    }
}
