fn main() {
    let input = include_str!("input.txt");

    let mut lines = input.lines();

    let mut crates = vec![];
    let mut crate_lines = vec![];

    for l in lines.by_ref() {
        if l.chars().any(|c| c.is_ascii_digit()) {
            let mut rev_line = l.chars().rev();
            rev_line.next();
            let n = rev_line.next().unwrap().to_digit(10).unwrap() as usize;
            crates = vec![vec![];n];
            break;
        }
        crate_lines.push(l);
    }
    lines.next();

    for cl in crate_lines.into_iter().rev() {
        let mut chars = cl.chars();
        chars.next();
        let mut i = 0;
        while let Some(c) = chars.next() {
            if !c.is_whitespace() {
                crates[i].push(c);
            }
            i += 1;
            chars.nth(2);
        }
    }

    for (i,c) in crates.clone().into_iter().enumerate() {
        println!("{}: {:?}", i+1, c);
    }

    for l in lines {
        let mut chars = l.chars();
        let mut mv = (chars.nth(5).unwrap().to_digit(10).unwrap()) as usize;
        if let Some(c) = chars.next() {
            if let Some(n) = c.to_digit(10) {
                mv *= 10;
                mv += n as usize;
                chars.next();
            }
        }
        let fm = (chars.nth(5).unwrap().to_digit(10).unwrap() - 1) as usize;
        let to = (chars.nth(4).unwrap().to_digit(10).unwrap() - 1) as usize;

        let smv = crates[fm].len().saturating_sub(mv);
        let mut append = crates.clone()[fm][smv..].to_vec();
        //append.reverse(); // Only for part 1
        crates[to].append(&mut append);
        crates[fm].truncate(smv);

        println!("{l} {:?}", (mv,fm,to));
        for (i,c) in crates.clone().into_iter().enumerate() {
            println!("{}: {:?}", i+1, c);
        }
    }

    println!();
    for c in crates {
        if let Some(last) = c.last() {
            print!("{last}");
        }
    }
    println!();    
}
