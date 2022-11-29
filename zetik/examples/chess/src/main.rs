use zetik::Chess;

fn main() {
    println!("{}", Chess::default().to_fen());
    println!("{}", Chess::default().to_fen_url('~'));
    Chess::default().pretty_print();
}
