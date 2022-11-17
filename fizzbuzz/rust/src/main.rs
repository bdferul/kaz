fn main() {
    let sneed = [
        ("Fizz", 3),
        ("Buzz", 5),
    ];

    for i in 1..=25 {
        let mut sneeded = false;
        for (s,x) in sneed {
            if i % x == 0 {
                print!("{s}");
                sneeded = true;
            }
        }

        println!("{}", if !sneeded {i.to_string()} else {"".to_string()});
    }
}
