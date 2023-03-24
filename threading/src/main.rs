use std::{
    sync::mpsc::channel,
    thread::{sleep, spawn},
    time::{Duration, Instant},
};

const OUTPUTS: [&str; 4] = ["one", "two", "three", "four"];

fn main() {
    for max_wait in 1..=4 {
        println!("max_wait: {max_wait}");
        let (p, c) = channel();

        spawn(move || {
            for o in OUTPUTS {
                let start = Instant::now();
                let sleep = Duration::from_millis(850);
                while Instant::now() - sleep < start {
                    if c.try_recv().is_ok() {
                        return;
                    }
                }
                println!("{o}")
            }
        });

        sleep(Duration::from_secs(max_wait));
        let status = if p.send(()).is_ok() {
            "Interrupted"
        } else {
            "Done!"
        };
        println!("{status}");
    }
}
