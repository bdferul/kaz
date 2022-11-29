use zetik::Chess;

fn main() {
    let d = Chess::default();
    println!("{}", d.to_fen());
    println!("{}", d.to_fen_url());
    d.pretty_print();
    
    let ff = Chess::from_fen("8/5k2/3p4/1p1Pp2p/pP2Pp1P/P4P1K/8/8 b - - 99 50".to_string()).unwrap();
    ff.pretty_print();
}
