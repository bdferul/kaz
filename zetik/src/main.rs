use zetik::{chess::Chess, mdx};

fn main() {
    let mut cb = Chess::from_fen("K1b58888888 b".to_string()).unwrap();
    cb.pretty_print();
    let _ = cb.mv(2, 9);
    cb.pretty_print();
}
