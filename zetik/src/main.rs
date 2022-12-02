use zetik::{chess::Chess, mdx};

fn main() {
    let src = mdx!(3, 7);
    let mut cb = zetik::chess::Chess::default();
    println!("hi");
    let bishop = cb.bishop_possible(src);
    println!("{:?}", bishop);
    for y in 0..8 {
        for x in 0..8 {
            let mut tail = if bishop.contains(&mdx!(x, y)) {
                "x"
            } else {
                " "
            };
            if src == mdx!(x, y) {
                tail = "o";
            }

            print!("{}{}", Chess::to_symbol(cb.board()[mdx!(x, y)], ' '), tail);
        }
        println!();
    }
}
