use std::{
    thread::sleep,
    time::{Duration, Instant},
};

const OUTPUTS: [&str; 4] = ["one", "two", "three", "four"];

fn main() {
    for max_wait in 1..=4 {
        let (p, c) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            for o in OUTPUTS {
                let start = Instant::now();
                let sleep = Duration::from_millis(850);
                while Instant::now() - sleep < start {
                    if c.try_recv().is_ok() {
                        return;
                    }
                }
                println!("{o}");
            }
        });

        sleep(Duration::from_secs(max_wait));
        match p.send(()) {
            Ok(_) => println!("Interrupted"),
            Err(_) => println!("Done!"),
        }
    }
}
