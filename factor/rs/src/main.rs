fn main() {
    println!("{:?}", (factor(120), prime_factor(120), prime_factor(7)));
}

fn factor(a: u32) -> Vec<u32> {
    let mut facts = vec![1];
    for i in 2..a {
        if i * 2 > a {
            break;
        }
        if a % i == 0 {
            facts.push(i);
        }
    }
    facts.push(a);
    facts
}

fn prime_factor(a: u32) -> Vec<u32> {
    let mut facts = vec![];
    let mut aa = a;
    for i in 2..a {
        if i*i > a {
            break;
        }

        while aa % i == 0 {
            aa /= i;
            facts.push(i);
        }
    }

    if facts.len() == 0 {
        facts.push(a);
    }

    facts
}