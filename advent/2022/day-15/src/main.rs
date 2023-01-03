// use rand::prelude::*;

fn main() {
    // let mut rng;
    let rs = vec!['c'; 7];
    for c in rs {
        println!("{c}: {:02}", rand::random::<u32>() % 100);
    }
}
